use crate::auth::password_generator::generate_password;
use crate::configuration::{MutConfiguration, StateConfiguration};
use crate::container::controller::DockerController;
use crate::data::data_const::get_config_file;
use tower_sessions::Session;

const SESSION_KEY: &str = "aio_authenticated";
const SESSION_DATE_KEY: &str = "date_time";

type Result<T> = std::result::Result<T, tower_sessions::session::Error>;

pub async fn set_auth_from_password(session: &Session, password: &str) -> Result<bool> {
    if !DockerController::is_login_allowed().await.unwrap() {
        Ok(false)
    } else {
        let value = StateConfiguration::test_password(password).await;
        session.insert(SESSION_KEY, value).await.and(Ok(value))
    }
}

pub async fn set_auth_from_token(session: &Session, token: &str) -> Result<bool> {
    let value = StateConfiguration::test_token(token).await;
    session.insert(SESSION_KEY, value).await.and(Ok(value))
}

pub async fn clear_auth(session: &Session) -> Result<()> {
    session.insert(SESSION_KEY, false).await
}

async fn set_auth_state(session: &Session, is_logged_in: bool) -> Result<()> {
    session.insert(SESSION_KEY, is_logged_in).await
    //     public function SetAuthState(bool $isLoggedIn) : void {
    //         if (!$this->IsAuthenticated() && $isLoggedIn === true) {
    //             $date = new DateTime();
    //             $dateTime = $date->getTimestamp();
    //             $_SESSION['date_time'] = $dateTime;
    //             $df = disk_free_space(DataConst::GetSessionDirectory());
    //             if ($df !== false && (int)$df < 10240) {
    //                 error_log(DataConst::GetSessionDirectory() . " has only less than 10KB free space. The login might not succeed because of that!");
    //             }
    //             file_put_contents(DataConst::GetSessionDateFile(), (string)$dateTime);
    //         }
    //         $_SESSION[self::SESSION_KEY] = $isLoggedIn;
    //     }
}
pub async fn is_authenticated(session: &Session) -> Result<bool> {
    session.get(SESSION_KEY).await.map(|v| v.unwrap_or(false))
}

pub fn can_be_installed() -> bool {
    !get_config_file().is_file()
}

pub async fn setup_password(mut config: MutConfiguration) -> Option<String> {
    if can_be_installed() {
        let password = generate_password(8);
        config.password = password.clone();
        config.commit();
        Some(password)
    } else {
        None
    }
}
