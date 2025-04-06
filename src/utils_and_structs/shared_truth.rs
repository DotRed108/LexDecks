// DECK META
pub const MAX_LEVELS: usize = 100;
pub const MAX_NOTE_FIELDS: usize = 150;
pub const TOKEN_COST_PER_KB: f64 = 0.00006818181;
pub const ALLOWED_UPLOAD_FILE_TYPES: [&str; 2] = [".apkg", ".csv"];
pub const RAW_DECK_SIZE_LIMIT: usize = 1000000000; // 1 GB

pub const DECK_ID_LENGTH: usize = 21;
pub const DECK_LIMIT: usize = 151;
pub const NOTE_LIMIT: usize = 100000;
pub const SEPARATOR: char = '\u{001F}';
pub const SEPARATOR2: char = '\u{001E}';
pub const SEPARATOR3: char = '\u{001D}';
pub const SEPARATOR4: char = '\u{001C}';
pub const SEPARATOR5: &str = "|\u{001F}|";
pub const MAX_ASSETS_PER_REQUEST: u8 = 25;

// Time
pub const ONE_MONTH_IN_SECONDS: u64 = 2629800;

// VERIFICATION
pub const PUBLIC_KEY: [u8; 32] = [183, 177, 157, 57, 78, 176, 181, 67, 152, 166, 91, 120, 67, 99, 14, 16, 189, 46, 30, 75, 88, 77, 182, 203, 206, 212, 82, 16, 179, 151, 71, 24];
pub const USER_CLAIM_SIGN_UP: &str = "wannabe_user";
pub const USER_CLAIM_REFRESH: &str = "refresh_user";
pub const USER_CLAIM_AUTH: &str = "user";
pub const IS_TRUSTED_CLAIM: &str = "trusted";
pub const AUTH_TOKEN_HEADER: &str = "token";
pub const MAX_EMAIL_SIZE: usize = 100;

// ASSET LINKS
pub const FULL_LOGO_PATH: &str = "../../images/Transparent_LexLingua_Primary Logo.png";
pub const LOGO_PATH: &str = "../../images/NavBarLogo.webp";
pub const CALENDAR_BG: &str = "../../images/starchart.avif";

// SIGN IN
pub const SIGN_IN_PAGE: &str = "http://localhost:1420/sign-in";

// KEYS
pub const LOCAL_AUTH_TOKEN_KEY: &str = "auth-token";
pub const LOCAL_REFRESH_TOKEN_KEY: &str = "refresh-token";
pub const LOCAL_USER_INFO_KEY: &str = "user-info";

