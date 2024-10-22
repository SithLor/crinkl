enum Rank {
    ADMIN,
    MODERATOR,
    HELPER,
    NORMAL,
}

enum PackageRank {
    MVP_PLUS,
    MVP,
    VIP_PLUS,
    VIP,
    NONE,
}

enum MonthlyPackageRank {
    SUPERSTAR,
    NONE,
}

struct PlayerData {
    success: bool,
}

struct Player {
    uuid: String,
    displayname: Option<String>,
    rank: Option<Rank>,
    packageRank: Option<PackageRank>,
    newPackageRank: Option<PackageRank>,
    monthlyPackageRank: Option<MonthlyPackageRank>,
    firstLogin: Option<u64>,
    lastLogin: Option<u64>,
    lastLogout: Option<u64>,
    stats: Option<serde_json::Value>, // Assuming stats is a JSON object
}


//300 per 5 minute
fn limit_requset(){

}

fn main() {
    let api_key = 
}

