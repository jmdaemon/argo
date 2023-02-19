// Program Constants
use const_format::formatcp;

pub const PROGRAM_NAME: &str        = "argo";
pub const VERSION: &str             = "0.1.0";
pub const AUTHOR: &str              = "Joseph Diza. <josephm.diza@gmail.com>";
pub const PROGRAM_DESCRIPTION: &str = "A next generation cross platform file manager";

pub const QUALIFIER: &str           = "com";
pub const ORGANIZATION: &str        = "jmdaemon";
pub const APPLICATION: &str         = PROGRAM_NAME;
pub const APP_ID: &str              = formatcp!("{}.{}.{}", QUALIFIER, ORGANIZATION, APPLICATION);
