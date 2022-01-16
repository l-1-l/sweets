#[derive(Debug, Clone)]
pub enum AccountStatus {
    DeregisteringBlack, // 账户同时进入注销期和小黑屋
    Deregistering,      // 账户进入注销期
    Frozen,             // 账户冻结
    Locked,             // 登录异常 账户锁定
    Stagged,            // 通过了注册验证, 但是没有完善信息
    ActiveBlack,        // 小黑屋
    Active,
}

impl AccountStatus {
    pub fn as_i16(&self) -> i16 {
        match self {
            AccountStatus::DeregisteringBlack => -4,
            AccountStatus::Deregistering => -3,
            AccountStatus::Frozen => -2,
            AccountStatus::Locked => -1,
            AccountStatus::Stagged => 0,
            AccountStatus::ActiveBlack => 1,
            AccountStatus::Active => 2,
        }
    }
}

impl From<i16> for AccountStatus {
    fn from(status: i16) -> Self {
        match status {
            -4 => AccountStatus::DeregisteringBlack,
            -3 => AccountStatus::Deregistering,
            -2 => AccountStatus::Frozen,
            0 => AccountStatus::Stagged,
            1 => AccountStatus::ActiveBlack,
            _ => AccountStatus::Active,
        }
    }
}

impl PartialEq for AccountStatus {
    fn eq(&self, other: &Self) -> bool {
        self.as_i16() == other.as_i16()
    }
}
