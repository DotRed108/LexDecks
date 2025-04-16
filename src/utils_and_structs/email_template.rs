use super::ui::{Color, Shadow};

const LAST_SIGN_IN: &str = "LAST_SIGN_IN";
const ACCOUNT_RANK: &str = "ACCOUNT_RANK";
const REDIRECT_LINK: &str = "REDIRECT_LINK";

pub enum EmailTemplate {
    SignUp,
    SignIn,
}
impl EmailTemplate {
    pub fn get_template(&self) -> String {
        match self {
            EmailTemplate::SignUp => format!(r#"
                <!doctype html>
                <html lang="en-US">

                <head>
                    <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
                    <title>New Account Email Template</title>
                    <meta name="description" content="New Account Email Template.">
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
                                            <img width="60" src="https://lexlingua.io/images/NavBarLogo.avif" title="logo" alt="logo">
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
                                                        <h1 style="color:#1e1e2d; font-weight:500; margin:0;font-size:32px;font-family:'Rubik',sans-serif;">Welcome to LexLingua
                                                        </h1>
                                                        <p style="font-size:15px; color:#455056; margin:8px 0 0; line-height:24px;">
                                                            Your sign up token has been generated. <br/>Clicking the link below will create your account, <br><strong>Something Something Something Something Something Something</strong>.</p>
                                                        <span
                                                            style="display:inline-block; vertical-align:middle; margin:29px 0 26px; border-bottom:1px solid #cecece; width:100px;"></span>
                                                        <p
                                                            style="color:#455056; font-size:18px;line-height:20px; margin:0; font-weight: 500;">
                                                            <strong
                                                                style="display: block;font-size: 13px; margin: 0 0 4px; color:rgba(0,0,0,.64); font-weight:normal;">Last Sign In</strong>{ACCOUNT_RANK}
                                                            <strong
                                                                style="display: block; font-size: 13px; margin: 24px 0 4px 0; font-weight:normal; color:rgba(0,0,0,.64);">Rank</strong>{LAST_SIGN_IN}
                                                        </p>

                                                        <a href="{REDIRECT_LINK}" class="button"
                                                            style="background:{winter3};text-decoration:none !important; box-shadow:{shadow}; border:0.2em solid {winter3}; max-width:400px; font-weight:600; margin-top:24px; color:#fff;text-transform:uppercase; font-size:1.1em;padding:0.3em;display:block;border-radius:3px;">Sign
                                                            Up</a>
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
            ),
            EmailTemplate::SignIn => format!(r#"
                <!doctype html>
                <html lang="en-US">

                <head>
                    <meta content="text/html; charset=utf-8" http-equiv="Content-Type" />
                    <title>New Account Email Template</title>
                    <meta name="description" content="New Account Email Template.">
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
                                            <img width="60" src="https://lexlingua.io/images/NavBarLogo.avif" title="logo" alt="logo">
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
                                                        <h1 style="color:#1e1e2d; font-weight:500; margin:0;font-size:32px;font-family:'Rubik',sans-serif;">Welcome back
                                                        </h1>
                                                        <p style="font-size:15px; color:#455056; margin:8px 0 0; line-height:24px;">
                                                            Your sign in token has been generated. <br/>Click the link below to sign in, <br><strong>Something Something Something Something Something Something</strong>.</p>
                                                        <span
                                                            style="display:inline-block; vertical-align:middle; margin:29px 0 26px; border-bottom:1px solid #cecece; width:100px;"></span>
                                                        <p
                                                            style="color:#455056; font-size:18px;line-height:20px; margin:0; font-weight: 500;">
                                                            <strong
                                                                style="display: block;font-size: 13px; margin: 0 0 4px; color:rgba(0,0,0,.64); font-weight:normal;">Last Sign In</strong>{ACCOUNT_RANK}
                                                            <strong
                                                                style="display: block; font-size: 13px; margin: 24px 0 4px 0; font-weight:normal; color:rgba(0,0,0,.64);">Rank</strong>{LAST_SIGN_IN}
                                                        </p>

                                                        <a href="{REDIRECT_LINK}" class="button"
                                                            style="background:{winter3};text-decoration:none !important; box-shadow:{shadow}; border:0.2em solid {winter3}; max-width:400px; font-weight:600; margin-top:24px; color:#fff;text-transform:uppercase; font-size:1.1em;padding:0.3em;display:block;border-radius:3px;">Sign
                                                            In</a>
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
            ),
        }
    }
}


pub const SIGN_IN_TEMPLATE: &str = "<h1>LexLingua Sign in link</h1>";

