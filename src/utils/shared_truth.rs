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
pub const ONE_DAY_IN_SECONDS: u64 = 86400;
pub const CACHE_OUT_OF_DATE_LIMIT: u64 = ONE_DAY_IN_SECONDS * 5;

// VERIFICATION
pub const PUBLIC_KEY: [u8; 32] = [224,221,70,136,138,4,23,242,133,57,200,126,219,223,19,130,157,157,198,186,206,254,54,38,191,215,226,51,244,191,74,177];
pub const USER_CLAIM_SIGN_UP: &str = "wannabe_user";
pub const USER_CLAIM_REFRESH: &str = "refresh_user";
pub const USER_CLAIM_AUTH: &str = "user";
pub const IS_TRUSTED_CLAIM: &str = "trusted";
pub const AUTH_TOKEN_HEADER: &str = "Authorization";
pub const MAX_EMAIL_SIZE: usize = 100;

// ASSET LINKS
pub const FULL_LOGO_PATH: &str = "../../images/MainLogo.avif";
pub const LOGO_PATH: &str = "../../images/NavBarLogo.avif";
pub const CALENDAR_BG: &str = "../../images/CalendarBG.avif";
pub const LESSONS_IMAGE: &str = "../../images/LessonsImage.avif";
pub const REVIEW_IMAGE: &str = "../../images/ReviewsImage.avif";

// SIGN IN
#[cfg(not(debug_assertions))]
pub const SIGN_IN_PAGE: &str = "https://lexlingua.io/sign-in";
#[cfg(debug_assertions)]
pub const SIGN_IN_PAGE: &str = "https://localhost:3000/sign-in";

// KEYS
pub const LOCAL_AUTH_TOKEN_KEY: &str = "auth-token";
pub const LOCAL_REFRESH_TOKEN_KEY: &str = "refresh-token";
pub const LOCAL_USER_INFO_KEY: &str = "user-info";
pub const CACHE_STATUS_COOKIE_KEY: &str = "cache-status";
pub const EXP_CLAIM_KEY: &str = "exp";
pub const EMAIL_CLAIM_KEY: &str = "user";

pub const S3_CREATION_DATE_URL_PARAM: &str = "X-Amz-Date=";
pub const S3_EXPIRATION_URL_PARAM: &str = "X-Amz-Expires=";
