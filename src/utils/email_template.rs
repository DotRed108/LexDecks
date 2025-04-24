use super::ui::{Color, Shadow};

const TEXT_LINE_1: &str = "TEXT_LINE_1";
const TEXT_LINE_2: &str = "TEXT_LINE_2";
const TEXT_LINE_3: &str = "TEXT_LINE_3";
const TITLE_TEXT: &str = "TITLE_TEXT";
const BUTTON_TEXT: &str = "BUTTON_TEXT";
const LOGO_LINK: &str = "https://lexlingua.io/images/NavBarLogo.avif";
pub const EMAIL_FIELD_1: &str = "EMAIL_FIELD_1";
pub const EMAIL_FIELD_1_VALUE: &str = "EMAIL_FIELD_1_VALUE";
pub const EMAIL_FIELD_2: &str = "EMAIL_FIELD_2";
pub const EMAIL_FIELD_2_VALUE: &str = "EMAIL_FIELD_2_VALUE";
pub const REDIRECT_LINK: &str = "REDIRECT_LINK";

pub enum EmailTemplate {
    SignUp,
    SignIn,
}
impl EmailTemplate {
    pub fn get_template(&self) -> String {
        let email_template = format!(r#"
            <!doctype html>
            <html lang="en-US">

            <head>
                <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
                <title>Sign Up/In Email Template</title>
                <meta name="description" content="Sign Up/In Email Template.">
                <style type="text/css">
                    a:hover {{text-decoration: underline !important;}}
                    .button:hover {{
                        border:0.2em solid {border_color};
                    }}
                </style>
            </head>

            <body marginheight="0" topmargin="0" marginwidth="0" style="margin: 0px; background-color: {color1};" leftmargin="0">
                <!-- 100% body table -->
                <table cellspacing="0" border="0" cellpadding="0" width="100%" bgcolor="{color1}"
                    style="@import url(https://fonts.googleapis.com/css?family=Rubik:300,400,500,700|Open+Sans:300,400,600,700); font-family: 'Open Sans', sans-serif;">
                    <tr>
                        <td>
                            <table style="background-color: {color1}; max-width:670px; margin:0 auto;" width="100%" border="0"
                                align="center" cellpadding="0" cellspacing="0">
                                <tr>
                                    <td style="height:80px;">&nbsp;</td>
                                </tr>
                                <tr>
                                    <td style="text-align:center;">
                                        <a href="https://lexlingua.io/" title="logo" target="_blank">
                                        <img width="100" src="{LOGO_LINK}" title="logo" alt="logo">
                                    </a>
                                    </td>
                                </tr>
                                <tr>
                                    <td style="height:20px;">&nbsp;</td>
                                </tr>
                                <tr>
                                    <td>
                                        <table width="95%" border="0" align="center" cellpadding="0" cellspacing="0"
                                            style="max-width:670px; background:#fff; border-radius:3px; text-align:center;-webkit-box-shadow:0 6px 18px 0 rgba(0,0,0,.06);-moz-box-shadow:0 6px 18px 0 rgba(0,0,0,.06);box-shadow:0 6px 18px 0 rgba(0,0,0,.06);">
                                            <tr>
                                                <td style="height:40px;">&nbsp;</td>
                                            </tr>
                                            <tr>
                                                <td style="padding:0 35px;">
                                                    <h1 style="color:#1e1e2d; font-weight:500; margin:0;font-size:32px;font-family:'Rubik',sans-serif;">{TITLE_TEXT}
                                                    </h1>
                                                    <p style="font-size:15px; color:#455056; margin:8px 0 0; line-height:24px;">
                                                        {TEXT_LINE_1} <br/>{TEXT_LINE_2} <br><strong>{TEXT_LINE_3}</strong>.</p>
                                                    <span
                                                        style="display:inline-block; vertical-align:middle; margin:29px 0 26px; border-bottom:1px solid #cecece; width:100px;"></span>
                                                    <p
                                                        style="color:#455056; font-size:18px;line-height:20px; margin:0; font-weight: 500;">
                                                        <strong
                                                            style="display: block;font-size: 13px; margin: 0 0 4px; color:rgba(0,0,0,.64); font-weight:normal;">{EMAIL_FIELD_1}</strong>{EMAIL_FIELD_1_VALUE}
                                                        <strong
                                                            style="display: block; font-size: 13px; margin: 24px 0 4px 0; font-weight:normal; color:rgba(0,0,0,.64);">{EMAIL_FIELD_2}</strong>{EMAIL_FIELD_2_VALUE}
                                                    </p>

                                                    <a class="button" href="{REDIRECT_LINK}"
                                                        style="background:{winter3};text-decoration:none !important; box-shadow:{shadow}; border:0.2em solid {winter3}; max-width: 350px; width: 100%; font-weight:600; margin-inline: auto; margin-top:24px; color:#fff; font-size:1.1em;padding:0.3em;display: inline-block;border-radius:3px;">{BUTTON_TEXT}
                                                        </a>
                                                </td>
                                            </tr>
                                            <tr>
                                                <td style="height:40px;">&nbsp;</td>
                                            </tr>
                                        </table>
                                    </td>
                                </tr>
                                <tr>
                                    <td style="height:20px;">&nbsp;</td>
                                </tr>
                                <tr>
                                    <td style="height:80px;">&nbsp;</td>
                                </tr>
                            </table>
                        </td>
                    </tr>
                </table>
                <!--/100% body table-->
            </body>

            </html>
            "#,
            color1 = "#f2f3f8",
            winter3 = Color::Winter3.hex(),
            border_color = Color::White.rgba(30),
            shadow = Shadow::dark().css(),
            // darkslate = Color::DarkSlate.hex(),
        );
        match self {
            EmailTemplate::SignUp => email_template
            .replace(TITLE_TEXT, "Welcome to LexLingua")
            .replace(TEXT_LINE_1, "Your sign up token has been generated.")
            .replace(TEXT_LINE_2, "Clicking the link below will create your account,")
            .replace(TEXT_LINE_3, "If you did not request this email please do not click the link")
            .replace(BUTTON_TEXT, "Sign Up"),
            EmailTemplate::SignIn => email_template
            .replace(TITLE_TEXT, "Welcome Back")
            .replace(TEXT_LINE_1, "Your sign in token has been generated.")
            .replace(TEXT_LINE_2, "Clicking the link below will sign you in,")
            .replace(TEXT_LINE_3, "If you did not request this email please inform support service@lexlingua.io")
            .replace(BUTTON_TEXT, "Sign In"),
        }
    }
}

