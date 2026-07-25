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

    pub fn is_ai(&self) -> bool {
        !matches!(
            self.get_kind(),
            BotKind::LinkPreview | BotKind::SearchEngine | BotKind::Generic
        )
    }
}
