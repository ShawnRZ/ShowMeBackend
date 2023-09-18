use log::debug;
use serde::{Deserialize, Serialize};
use std::{fmt, mem::MaybeUninit, sync::Once};
use sysinfo::{ProcessExt, System, SystemExt};
use tokio::sync::RwLock;

type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(Debug, Serialize, Deserialize)]
pub struct Parameter {
    pub port: String,
    pub token: String,
    pub region: String,
}

impl Parameter {
    pub async fn refresh() -> Result<(), Error> {
        debug!("Parameter::refresh()");
        let mut sys = System::new_all();
        sys.refresh_all();
        let processes: Vec<_> = sys.processes_by_exact_name("LeagueClientUx.exe").collect();
        if processes.len() > 1 {
            Err(ParameterError::MultipleClientProcessesFound)?
        } else if processes.len() < 1 {
            Err(ParameterError::ClientProcessNotFound)?
        }
        let process = processes[0];
        debug!("{:#?}", process.cmd());
        let args = process.cmd();

        let port = args
            .iter()
            .find(|arg| arg.starts_with("--app-port"))
            .map(|arg| arg.strip_prefix("--app-port=").unwrap())
            .ok_or(ParameterError::PortNotFound)?
            .to_string();
        let token = args
            .iter()
            .find(|arg| arg.starts_with("--remoting-auth-token"))
            .map(|arg| arg.strip_prefix("--remoting-auth-token=").unwrap())
            .ok_or(ParameterError::TokenNotFound)?
            .to_string();
        let region = args
            .iter()
            .find(|arg| arg.starts_with("--rso_platform_id"))
            .map(|arg| arg.strip_prefix("--rso_platform_id=").unwrap())
            .ok_or(ParameterError::TokenNotFound)?
            .to_string();
        let mut g = GlobalParameter::get().write().await;
        g.para.port = port;
        g.para.token = token;
        g.para.region = region;
        g.valid = true;
        Ok(())
    }

    pub async fn get() -> Result<Parameter, Error> {
        debug!("Parameter::get()");
        let g = GlobalParameter::get().read().await;
        if !g.valid {
            Err(ParameterError::Unvalid)?
        }

        Ok(Parameter {
            port: g.para.port.to_string(),
            token: g.para.token.to_string(),
            region: g.para.region.to_string(),
        })
    }

    pub async fn abandon() -> Result<(), Error> {
        let mut gp = GlobalParameter::get().write().await;
        gp.valid = false;
        Ok(())
    }
}

struct GlobalParameter {
    para: Parameter,
    valid: bool,
}

impl GlobalParameter {
    fn get() -> &'static RwLock<GlobalParameter> {
        debug!("GlobalParameter::get()");
        static mut G_PARAMETER: MaybeUninit<RwLock<GlobalParameter>> = MaybeUninit::uninit();
        static ONCE: Once = Once::new();

        ONCE.call_once(|| unsafe {
            debug!("初始化LCU_PARAMETER");
            G_PARAMETER.as_mut_ptr().write(RwLock::new(GlobalParameter {
                para: Parameter {
                    port: String::new(),
                    token: String::new(),
                    region: String::new(),
                },
                valid: true,
            }));
        });
        unsafe { &*G_PARAMETER.as_ptr() }
    }
}

#[derive(Debug)]
pub enum ParameterError {
    // 存在多个客户端进程
    MultipleClientProcessesFound,
    // 没有找到客户端进程
    ClientProcessNotFound,
    // 没有找到端口
    PortNotFound,
    // 没有找到令牌
    TokenNotFound,
    // 无效参数
    Unvalid,
}

impl fmt::Display for ParameterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MultipleClientProcessesFound => write!(f, "存在多个客户端进程"),
            Self::ClientProcessNotFound => write!(f, "没有找到客户端进程"),
            Self::PortNotFound => write!(f, "没有找到端口"),
            Self::TokenNotFound => write!(f, "没有找到令牌"),
            Self::Unvalid => write!(f, "无效参数"),
        }
    }
}

impl std::error::Error for ParameterError {}
