use r2d2_sqlite::rusqlite::{
    self, ToSql,
    types::{FromSql, FromSqlError, FromSqlResult, ToSqlOutput, ValueRef},
};
use serde_derive::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum BotType {
    IMessage,
    FB,
    Twitter,
    Google,
    Bing,
    DuckDuckGo,
    Yandex,
    Baidu,
    Slack,
    Discord,
    Telegram,
    WhatsApp,
    LinkedIn,
    Pinterest,
    Reddit,
    Apple, // Apple's *search* crawler, distinct from iMessage preview
    Crawler,
    Spider,
    //  OpenAI
    Gptbot,       // GPTBot        — training
    OaiSearchBot, // OAI-SearchBot — AI search index (SearchGPT)
    ChatGptUser,  // ChatGPT-User  — user-triggered browse
    //  Anthropic
    ClaudeBot,       // ClaudeBot     — training
    ClaudeSearchBot, // Claude-SearchBot — retrieval for citations
    ClaudeUser,      // Claude-User   — user-triggered browse
    AnthropicAi,     // anthropic-ai  — legacy training token
    //  Perplexity
    PerplexityBot,  // index builder
    PerplexityUser, // human-triggered visit
    //  Google (AI tokens, distinct from Googlebot search)
    GoogleExtended, // Google-Extended — Gemini/Vertex training opt-out token
    GoogleOther,    // GoogleOther     — R&D / non-search fetch
    //  Apple (AI token, distinct from Applebot search)
    ApplebotExtended, // Applebot-Extended — Apple Intelligence training opt-out
    //  Others
    Amazonbot,         // Alexa/AI training
    MetaExternalAgent, // Meta-ExternalAgent / meta-externalfetcher — Llama
    FacebookBot,       // FacebookBot — Meta AI training (≠ facebookexternalhit preview)
    Bytespider,        // ByteDance — training, notoriously ignores robots.txt
    CCBot,             // Common Crawl — feeds most LLM training corpora
    DuckAssistBot,     // DuckDuckGo AI assist retrieval
    CohereAi,          // cohere-ai
    Diffbot,           // Diffbot — structured extraction, mixed compliance
    Other,             // generic "bot"
}

impl BotType {
    pub fn as_str(&self) -> &'static str {
        match self {
            BotType::IMessage => "iMessage",
            BotType::FB => "Facebook",
            BotType::Twitter => "Twitter/X",
            BotType::Google => "Google",
            BotType::Bing => "Bing",
            BotType::DuckDuckGo => "DuckDuckGo",
            BotType::Yandex => "Yandex",
            BotType::Baidu => "Baidu",
            BotType::Slack => "Slack",
            BotType::Discord => "Discord",
            BotType::Telegram => "Telegram",
            BotType::WhatsApp => "WhatsApp",
            BotType::LinkedIn => "LinkedIn",
            BotType::Pinterest => "Pinterest",
            BotType::Reddit => "Reddit",
            BotType::Apple => "Applebot",
            BotType::Crawler => "Crawler",
            BotType::Spider => "Spider",
            BotType::Gptbot => "GPTBot",
            BotType::OaiSearchBot => "OAI-SearchBot",
            BotType::ChatGptUser => "ChatGPT-User",
            BotType::ClaudeBot => "ClaudeBot",
            BotType::ClaudeSearchBot => "Claude-SearchBot",
            BotType::ClaudeUser => "Claude-User",
            BotType::AnthropicAi => "anthropic-ai",
            BotType::PerplexityBot => "PerplexityBot",
            BotType::PerplexityUser => "Perplexity-User",
            BotType::GoogleExtended => "Google-Extended",
            BotType::GoogleOther => "GoogleOther",
            BotType::ApplebotExtended => "Applebot-Extended",
            BotType::Amazonbot => "Amazonbot",
            BotType::MetaExternalAgent => "Meta-ExternalAgent",
            BotType::FacebookBot => "FacebookBot",
            BotType::Bytespider => "Bytespider",
            BotType::CCBot => "CCBot",
            BotType::DuckAssistBot => "DuckAssistBot",
            BotType::CohereAi => "cohere-ai",
            BotType::Diffbot => "Diffbot",
            BotType::Other => "Bot",
        }
    }
}

impl fmt::Display for BotType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Classify a user-agent string. Returns `None` if it doesn't look like a bot.
pub fn get_bot_type(ua: &str) -> Option<BotType> {
    let ua = ua.to_ascii_lowercase();
    let has = |needle: &str| ua.contains(needle);

    // --- Most specific first ---

    // iMessage / Apple link-preview fetcher.
    // Apple bundles facebookexternalhit + Facebot + Twitterbot in one UA, e.g.:
    //   "... Safari/601.2.4 facebookexternalhit/1.1 Facebot Twitterbot/1.0"
    // The co-occurrence of facebookexternalhit AND twitterbot is the tell:
    // real Facebook never sends "twitterbot".
    if has("facebookexternalhit") && has("twitterbot") {
        return Some(BotType::IMessage);
    }

    // Telegram identifies as "TelegramBot (like TwitterBot)" — catch before Twitter.
    if has("telegrambot") || has("telegram") {
        return Some(BotType::Telegram);
    }

    if has("twitterbot") {
        return Some(BotType::Twitter);
    }
    if has("facebookexternalhit") || has("facebookcatalog") || has("facebot") {
        return Some(BotType::FB);
    }
    if has("slackbot") || has("slack-imgproxy") {
        return Some(BotType::Slack);
    }
    if has("discordbot") {
        return Some(BotType::Discord);
    }
    if has("whatsapp") {
        return Some(BotType::WhatsApp);
    }
    if has("linkedinbot") {
        return Some(BotType::LinkedIn);
    }
    if has("pinterest") {
        return Some(BotType::Pinterest);
    }
    if has("redditbot") {
        return Some(BotType::Reddit);
    }

    // Search-engine crawlers (specific names before generic bot/spider).
    if has("googlebot")
        || has("adsbot-google")
        || has("mediapartners-google")
        || has("google-inspectiontool")
        || has("storebot-google")
    {
        return Some(BotType::Google);
    }
    if has("bingbot") || has("bingpreview") || has("adidxbot") || has("msnbot") {
        return Some(BotType::Bing);
    }
    if has("duckduckbot") || has("duckduckgo") {
        return Some(BotType::DuckDuckGo);
    }
    if has("yandexbot") || has("yandex") {
        return Some(BotType::Yandex);
    }
    if has("baiduspider") {
        // contains "spider" — must precede the fallback
        return Some(BotType::Baidu);
    }

    // Apple's actual search crawler (NOT the iMessage preview fetcher).
    if has("applebot") {
        return Some(BotType::Apple);
    }

    // --- Generic fallbacks ---
    if has("crawler") {
        return Some(BotType::Crawler);
    }
    if has("spider") {
        return Some(BotType::Spider);
    }

    // ============ AI crawlers / agents (2026) ============
    // Order note: -Extended tokens and Bytespider must precede their
    // search-engine / generic-spider counterparts below.

    // OpenAI
    if has("oai-searchbot") {
        return Some(BotType::OaiSearchBot);
    }
    if has("chatgpt-user") {
        return Some(BotType::ChatGptUser);
    }
    if has("gptbot") {
        return Some(BotType::Gptbot);
    }

    // Anthropic
    if has("claude-searchbot") {
        return Some(BotType::ClaudeSearchBot);
    }
    if has("claude-user") {
        return Some(BotType::ClaudeUser);
    }
    if has("claudebot") || has("claude-web") {
        return Some(BotType::ClaudeBot);
    }
    if has("anthropic-ai") {
        return Some(BotType::AnthropicAi);
    }

    // Perplexity
    if has("perplexity-user") {
        return Some(BotType::PerplexityUser);
    }
    if has("perplexitybot") {
        return Some(BotType::PerplexityBot);
    }

    // Google AI tokens (must come before the googlebot search check)
    if has("google-extended") {
        return Some(BotType::GoogleExtended);
    }
    if has("google-cloudvertexbot") {
        return Some(BotType::GoogleExtended);
    }
    if has("googleother") {
        return Some(BotType::GoogleOther);
    }

    // Apple AI token (MUST precede the plain `applebot` search check)
    if has("applebot-extended") {
        return Some(BotType::ApplebotExtended);
    }

    // Meta AI (≠ facebookexternalhit link preview, handled earlier)
    if has("meta-externalagent") || has("meta-externalfetcher") {
        return Some(BotType::MetaExternalAgent);
    }
    if has("facebookbot") {
        return Some(BotType::FacebookBot);
    }

    // Others
    if has("amazonbot") {
        return Some(BotType::Amazonbot);
    }
    if has("bytespider") {
        return Some(BotType::Bytespider);
    } // before `spider`
    if has("ccbot") {
        return Some(BotType::CCBot);
    }
    if has("duckassistbot") {
        return Some(BotType::DuckAssistBot);
    }
    if has("cohere-ai") {
        return Some(BotType::CohereAi);
    }
    if has("diffbot") {
        return Some(BotType::Diffbot);
    }

    if has("bot") {
        return Some(BotType::Other);
    }

    None
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum BotKind {
    LinkPreview,  // unfurls a URL into a card (iMessage, Slack, Discord, FB, X…)
    SearchEngine, // classic web index (Google, Bing, Yandex…)
    AiTraining,   // bulk corpus collection for model training
    AiSearch,     // real-time retrieval that can cite you in AI answers
    AiAgent,      // a human asked their AI assistant to fetch this URL
    Generic,      // unclassified crawler/spider/bot
}

impl BotType {
    pub fn get_kind(&self) -> BotKind {
        use BotType::*;
        match self {
            IMessage | FB | Twitter | Slack | Discord | Telegram | WhatsApp | LinkedIn
            | Pinterest | Reddit => BotKind::LinkPreview,

            Google | Bing | DuckDuckGo | Yandex | Baidu | Apple => BotKind::SearchEngine,

            Gptbot | ClaudeBot | AnthropicAi | GoogleExtended | ApplebotExtended | Amazonbot
            | MetaExternalAgent | FacebookBot | Bytespider | CCBot | Diffbot | CohereAi => {
                BotKind::AiTraining
            }

            OaiSearchBot | ClaudeSearchBot | PerplexityBot | DuckAssistBot | GoogleOther => {
                BotKind::AiSearch
            }

            ChatGptUser | ClaudeUser | PerplexityUser => BotKind::AiAgent,

            Crawler | Spider | Other => BotKind::Generic,
        }
    }

    pub fn _is_ai(&self) -> bool {
        !matches!(
            self.get_kind(),
            BotKind::LinkPreview | BotKind::SearchEngine | BotKind::Generic
        )
    }
}

impl BotKind {
    pub const fn as_str(self) -> &'static str {
        match self {
            BotKind::LinkPreview => "link preview",
            BotKind::SearchEngine => "search engine",
            BotKind::AiTraining => "ai training",
            BotKind::AiSearch => "ai search",
            BotKind::AiAgent => "ai agent",
            BotKind::Generic => "generic",
        }
    }
}

impl ToSql for BotKind {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for BotKind {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value.as_str()? {
            "link_preview" => Ok(BotKind::LinkPreview),
            "search_engine" => Ok(BotKind::SearchEngine),
            "ai_training" => Ok(BotKind::AiTraining),
            "ai_search" => Ok(BotKind::AiSearch),
            "ai_agent" => Ok(BotKind::AiAgent),
            "generic" => Ok(BotKind::Generic),
            other => Err(FromSqlError::Other(
                format!("unknown BotKind: {other}").into(),
            )),
        }
    }
}

impl ToSql for BotType {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        Ok(ToSqlOutput::from(self.as_str()))
    }
}

impl FromSql for BotType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        BotType::from_str_name(value.as_str()?)
            .ok_or_else(|| FromSqlError::Other("unknown BotType".into()))
    }
}

impl BotType {
    pub fn from_str_name(s: &str) -> Option<Self> {
        Some(match s {
            "iMessage" => Self::IMessage,
            "Facebook" => Self::FB,
            "Twitter/X" => Self::Twitter,
            "Google" => Self::Google,
            "Bing" => Self::Bing,
            "DuckDuckGo" => Self::DuckDuckGo,
            "Yandex" => Self::Yandex,
            "Baidu" => Self::Baidu,
            "Slack" => Self::Slack,
            "Discord" => Self::Discord,
            "Telegram" => Self::Telegram,
            "WhatsApp" => Self::WhatsApp,
            "LinkedIn" => Self::LinkedIn,
            "Pinterest" => Self::Pinterest,
            "Reddit" => Self::Reddit,
            "Applebot" => Self::Apple,
            "Crawler" => Self::Crawler,
            "Spider" => Self::Spider,
            "GPTBot" => Self::Gptbot,
            "OAI-SearchBot" => Self::OaiSearchBot,
            "ChatGPT-User" => Self::ChatGptUser,
            "ClaudeBot" => Self::ClaudeBot,
            "Claude-SearchBot" => Self::ClaudeSearchBot,
            "Claude-User" => Self::ClaudeUser,
            "anthropic-ai" => Self::AnthropicAi,
            "PerplexityBot" => Self::PerplexityBot,
            "Perplexity-User" => Self::PerplexityUser,
            "Google-Extended" => Self::GoogleExtended,
            "GoogleOther" => Self::GoogleOther,
            "Applebot-Extended" => Self::ApplebotExtended,
            "Amazonbot" => Self::Amazonbot,
            "Meta-ExternalAgent" => Self::MetaExternalAgent,
            "FacebookBot" => Self::FacebookBot,
            "Bytespider" => Self::Bytespider,
            "CCBot" => Self::CCBot,
            "DuckAssistBot" => Self::DuckAssistBot,
            "cohere-ai" => Self::CohereAi,
            "Diffbot" => Self::Diffbot,
            "Bot" => Self::Other,
            _ => return None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_claudebot_ua() {
        let ua = "Mozilla/5.0 AppleWebKit/537.36 (KHTML, like Gecko; compatible; ClaudeBot/1.0; +claudebot@anthropic.com)";

        let bot = get_bot_type(ua).expect("should be detected as a bot");
        assert_eq!(bot, BotType::ClaudeBot);
        assert_eq!(bot.as_str(), "ClaudeBot");
        assert_eq!(bot.get_kind(), BotKind::AiTraining);
        assert!(bot._is_ai());
    }
}
