/**
 * Copyright GLIDE-for-Redis Project Contributors - SPDX Identifier: Apache-2.0
 */
use redis::{cmd, Cmd};

#[cfg(feature = "socket-layer")]
use crate::redis_request::RequestType as ProtobufRequestType;

#[repr(C)]
#[derive(Debug)]
pub enum RequestType {
    InvalidRequest = 0,
    CustomCommand = 1,
    Get = 2,
    Set = 3,
    Ping = 4,
    Info = 5,
    Del = 6,
    Select = 7,
    ConfigGet = 8,
    ConfigSet = 9,
    ConfigResetStat = 10,
    ConfigRewrite = 11,
    ClientGetName = 12,
    ClientGetRedir = 13,
    ClientId = 14,
    ClientInfo = 15,
    ClientKill = 16,
    ClientList = 17,
    ClientNoEvict = 18,
    ClientNoTouch = 19,
    ClientPause = 20,
    ClientReply = 21,
    ClientSetInfo = 22,
    ClientSetName = 23,
    ClientUnblock = 24,
    ClientUnpause = 25,
    Expire = 26,
    HSet = 27,
    HGet = 28,
    HDel = 29,
    HExists = 30,
    MGet = 31,
    MSet = 32,
    Incr = 33,
    IncrBy = 34,
    Decr = 35,
    IncrByFloat = 36,
    DecrBy = 37,
    HGetAll = 38,
    HMSet = 39,
    HMGet = 40,
    HIncrBy = 41,
    HIncrByFloat = 42,
    LPush = 43,
    LPop = 44,
    RPush = 45,
    RPop = 46,
    LLen = 47,
    LRem = 48,
    LRange = 49,
    LTrim = 50,
    SAdd = 51,
    SRem = 52,
    SMembers = 53,
    SCard = 54,
    PExpireAt = 55,
    PExpire = 56,
    ExpireAt = 57,
    Exists = 58,
    Unlink = 59,
    TTL = 60,
    ZAdd = 61,
    ZRem = 62,
    ZRange = 63,
    ZCard = 64,
    ZCount = 65,
    ZIncrBy = 66,
    ZScore = 67,
    Type = 68,
    HLen = 69,
    Echo = 70,
    ZPopMin = 71,
    Strlen = 72,
    LIndex = 73,
    ZPopMax = 74,
    XRead = 75,
    XAdd = 76,
    XReadGroup = 77,
    XAck = 78,
    XTrim = 79,
    XGroupCreate = 80,
    XGroupDestroy = 81,
    HSetNX = 82,
    SIsMember = 83,
    HVals = 84,
    PTTL = 85,
    ZRemRangeByRank = 86,
    Persist = 87,
    ZRemRangeByScore = 88,
    Time = 89,
    ZRank = 90,
    Rename = 91,
    DBSize = 92,
    BRPop = 93,
    HKeys = 94,
    SPop = 95,
    PfAdd = 96,
    PfCount = 97,
    PfMerge = 98,
    BLPop = 100,
    LInsert = 101,
    RPushX = 102,
    LPushX = 103,
    ZMScore = 104,
    ZDiff = 105,
    ZDiffStore = 106,
    SetRange = 107,
    ZRemRangeByLex = 108,
    ZLexCount = 109,
    Append = 110,
    SUnionStore = 111,
    SDiffStore = 112,
    SInter = 113,
    SInterStore = 114,
    ZRangeStore = 115,
    GetRange = 116,
    SMove = 117,
    SMIsMember = 118,
    ZUnionStore = 119,
    LastSave = 120,
    GeoAdd = 121,
    GeoHash = 122,
    ObjectEncoding = 123,
    SDiff = 124,
    ObjectIdleTime = 125,
    ObjectRefCount = 126,
    Lolwut = 100500,
    GeoDist = 127,
    GeoPos = 128,
    BZPopMax = 129,
    ObjectFreq = 130,
    RenameNX = 131,
    Touch = 132,
    ZRevRank = 133,
    ZInterStore = 134,
    HRandField = 135,
    ZUnion = 136,
    BZPopMin = 137,
    FlushAll = 138,
    ZRandMember = 139,
    Bitcount = 140,
    BZMPop = 141,
    ZInterCard = 143,
    ZMPop = 144,
    FunctionLoad = 150,
}

fn get_two_word_command(first: &str, second: &str) -> Cmd {
    let mut cmd = cmd(first);
    cmd.arg(second);
    cmd
}

#[cfg(feature = "socket-layer")]
impl From<::protobuf::EnumOrUnknown<ProtobufRequestType>> for RequestType {
    fn from(value: ::protobuf::EnumOrUnknown<ProtobufRequestType>) -> Self {
        match value.enum_value_or(ProtobufRequestType::InvalidRequest) {
            ProtobufRequestType::InvalidRequest => RequestType::InvalidRequest,
            ProtobufRequestType::CustomCommand => RequestType::CustomCommand,
            ProtobufRequestType::Get => RequestType::Get,
            ProtobufRequestType::Set => RequestType::Set,
            ProtobufRequestType::Ping => RequestType::Ping,
            ProtobufRequestType::Info => RequestType::Info,
            ProtobufRequestType::Del => RequestType::Del,
            ProtobufRequestType::Select => RequestType::Select,
            ProtobufRequestType::ConfigGet => RequestType::ConfigGet,
            ProtobufRequestType::ConfigSet => RequestType::ConfigSet,
            ProtobufRequestType::ConfigResetStat => RequestType::ConfigResetStat,
            ProtobufRequestType::ConfigRewrite => RequestType::ConfigRewrite,
            ProtobufRequestType::ClientGetName => RequestType::ClientGetName,
            ProtobufRequestType::ClientGetRedir => RequestType::ClientGetRedir,
            ProtobufRequestType::ClientId => RequestType::ClientId,
            ProtobufRequestType::ClientInfo => RequestType::ClientInfo,
            ProtobufRequestType::ClientKill => RequestType::ClientKill,
            ProtobufRequestType::ClientList => RequestType::ClientList,
            ProtobufRequestType::ClientNoEvict => RequestType::ClientNoEvict,
            ProtobufRequestType::ClientNoTouch => RequestType::ClientNoTouch,
            ProtobufRequestType::ClientPause => RequestType::ClientPause,
            ProtobufRequestType::ClientReply => RequestType::ClientReply,
            ProtobufRequestType::ClientSetInfo => RequestType::ClientSetInfo,
            ProtobufRequestType::ClientSetName => RequestType::ClientSetName,
            ProtobufRequestType::ClientUnblock => RequestType::ClientUnblock,
            ProtobufRequestType::ClientUnpause => RequestType::ClientUnpause,
            ProtobufRequestType::Expire => RequestType::Expire,
            ProtobufRequestType::HSet => RequestType::HSet,
            ProtobufRequestType::HGet => RequestType::HGet,
            ProtobufRequestType::HDel => RequestType::HDel,
            ProtobufRequestType::HExists => RequestType::HExists,
            ProtobufRequestType::MSet => RequestType::MSet,
            ProtobufRequestType::MGet => RequestType::MGet,
            ProtobufRequestType::Incr => RequestType::Incr,
            ProtobufRequestType::IncrBy => RequestType::IncrBy,
            ProtobufRequestType::IncrByFloat => RequestType::IncrByFloat,
            ProtobufRequestType::Decr => RequestType::Decr,
            ProtobufRequestType::DecrBy => RequestType::DecrBy,
            ProtobufRequestType::HGetAll => RequestType::HGetAll,
            ProtobufRequestType::HMSet => RequestType::HMSet,
            ProtobufRequestType::HMGet => RequestType::HMGet,
            ProtobufRequestType::HIncrBy => RequestType::HIncrBy,
            ProtobufRequestType::HIncrByFloat => RequestType::HIncrByFloat,
            ProtobufRequestType::LPush => RequestType::LPush,
            ProtobufRequestType::LPop => RequestType::LPop,
            ProtobufRequestType::RPush => RequestType::RPush,
            ProtobufRequestType::RPop => RequestType::RPop,
            ProtobufRequestType::LLen => RequestType::LLen,
            ProtobufRequestType::LRem => RequestType::LRem,
            ProtobufRequestType::LRange => RequestType::LRange,
            ProtobufRequestType::LTrim => RequestType::LTrim,
            ProtobufRequestType::SAdd => RequestType::SAdd,
            ProtobufRequestType::SRem => RequestType::SRem,
            ProtobufRequestType::SMembers => RequestType::SMembers,
            ProtobufRequestType::SCard => RequestType::SCard,
            ProtobufRequestType::PExpireAt => RequestType::PExpireAt,
            ProtobufRequestType::PExpire => RequestType::PExpire,
            ProtobufRequestType::ExpireAt => RequestType::ExpireAt,
            ProtobufRequestType::Exists => RequestType::Exists,
            ProtobufRequestType::Unlink => RequestType::Unlink,
            ProtobufRequestType::TTL => RequestType::TTL,
            ProtobufRequestType::ZAdd => RequestType::ZAdd,
            ProtobufRequestType::ZRem => RequestType::ZRem,
            ProtobufRequestType::ZRange => RequestType::ZRange,
            ProtobufRequestType::ZCard => RequestType::ZCard,
            ProtobufRequestType::ZCount => RequestType::ZCount,
            ProtobufRequestType::ZIncrBy => RequestType::ZIncrBy,
            ProtobufRequestType::ZScore => RequestType::ZScore,
            ProtobufRequestType::Type => RequestType::Type,
            ProtobufRequestType::HLen => RequestType::HLen,
            ProtobufRequestType::Echo => RequestType::Echo,
            ProtobufRequestType::ZPopMin => RequestType::ZPopMin,
            ProtobufRequestType::Strlen => RequestType::Strlen,
            ProtobufRequestType::LIndex => RequestType::LIndex,
            ProtobufRequestType::ZPopMax => RequestType::ZPopMax,
            ProtobufRequestType::XAck => RequestType::XAck,
            ProtobufRequestType::XAdd => RequestType::XAdd,
            ProtobufRequestType::XReadGroup => RequestType::XReadGroup,
            ProtobufRequestType::XRead => RequestType::XRead,
            ProtobufRequestType::XGroupCreate => RequestType::XGroupCreate,
            ProtobufRequestType::XGroupDestroy => RequestType::XGroupDestroy,
            ProtobufRequestType::XTrim => RequestType::XTrim,
            ProtobufRequestType::HSetNX => RequestType::HSetNX,
            ProtobufRequestType::SIsMember => RequestType::SIsMember,
            ProtobufRequestType::HVals => RequestType::HVals,
            ProtobufRequestType::PTTL => RequestType::PTTL,
            ProtobufRequestType::ZRemRangeByRank => RequestType::ZRemRangeByRank,
            ProtobufRequestType::Persist => RequestType::Persist,
            ProtobufRequestType::ZRemRangeByScore => RequestType::ZRemRangeByScore,
            ProtobufRequestType::Time => RequestType::Time,
            ProtobufRequestType::ZRank => RequestType::ZRank,
            ProtobufRequestType::Rename => RequestType::Rename,
            ProtobufRequestType::DBSize => RequestType::DBSize,
            ProtobufRequestType::BRPop => RequestType::BRPop,
            ProtobufRequestType::HKeys => RequestType::HKeys,
            ProtobufRequestType::PfAdd => RequestType::PfAdd,
            ProtobufRequestType::PfCount => RequestType::PfCount,
            ProtobufRequestType::PfMerge => RequestType::PfMerge,
            ProtobufRequestType::RPushX => RequestType::RPushX,
            ProtobufRequestType::LPushX => RequestType::LPushX,
            ProtobufRequestType::BLPop => RequestType::BLPop,
            ProtobufRequestType::LInsert => RequestType::LInsert,
            ProtobufRequestType::SPop => RequestType::SPop,
            ProtobufRequestType::ZMScore => RequestType::ZMScore,
            ProtobufRequestType::ZDiff => RequestType::ZDiff,
            ProtobufRequestType::ZDiffStore => RequestType::ZDiffStore,
            ProtobufRequestType::SetRange => RequestType::SetRange,
            ProtobufRequestType::ZRemRangeByLex => RequestType::ZRemRangeByLex,
            ProtobufRequestType::ZLexCount => RequestType::ZLexCount,
            ProtobufRequestType::Append => RequestType::Append,
            ProtobufRequestType::SDiffStore => RequestType::SDiffStore,
            ProtobufRequestType::SInter => RequestType::SInter,
            ProtobufRequestType::SInterStore => RequestType::SInterStore,
            ProtobufRequestType::SUnionStore => RequestType::SUnionStore,
            ProtobufRequestType::ZRangeStore => RequestType::ZRangeStore,
            ProtobufRequestType::GetRange => RequestType::GetRange,
            ProtobufRequestType::SMove => RequestType::SMove,
            ProtobufRequestType::SMIsMember => RequestType::SMIsMember,
            ProtobufRequestType::ZUnionStore => RequestType::ZUnionStore,
            ProtobufRequestType::LastSave => RequestType::LastSave,
            ProtobufRequestType::GeoAdd => RequestType::GeoAdd,
            ProtobufRequestType::GeoHash => RequestType::GeoHash,
            ProtobufRequestType::ObjectEncoding => RequestType::ObjectEncoding,
            ProtobufRequestType::ObjectFreq => RequestType::ObjectFreq,
            ProtobufRequestType::ObjectIdleTime => RequestType::ObjectIdleTime,
            ProtobufRequestType::GeoDist => RequestType::GeoDist,
            ProtobufRequestType::SDiff => RequestType::SDiff,
            ProtobufRequestType::ObjectRefCount => RequestType::ObjectRefCount,
            ProtobufRequestType::Lolwut => RequestType::Lolwut,
            ProtobufRequestType::GeoPos => RequestType::GeoPos,
            ProtobufRequestType::BZPopMax => RequestType::BZPopMax,
            ProtobufRequestType::RenameNX => RequestType::RenameNX,
            ProtobufRequestType::Touch => RequestType::Touch,
            ProtobufRequestType::ZRevRank => RequestType::ZRevRank,
            ProtobufRequestType::ZInterStore => RequestType::ZInterStore,
            ProtobufRequestType::HRandField => RequestType::HRandField,
            ProtobufRequestType::ZUnion => RequestType::ZUnion,
            ProtobufRequestType::BZPopMin => RequestType::BZPopMin,
            ProtobufRequestType::FlushAll => RequestType::FlushAll,
            ProtobufRequestType::ZRandMember => RequestType::ZRandMember,
            ProtobufRequestType::Bitcount => RequestType::Bitcount,
            ProtobufRequestType::BZMPop => RequestType::BZMPop,
            ProtobufRequestType::ZInterCard => RequestType::ZInterCard,
            ProtobufRequestType::ZMPop => RequestType::ZMPop,
            ProtobufRequestType::FunctionLoad => RequestType::FunctionLoad,
        }
    }
}

impl RequestType {
    /// Returns a `Cmd` set with the command name matching the request.
    pub fn get_command(&self) -> Option<Cmd> {
        match self {
            RequestType::InvalidRequest => None,
            RequestType::CustomCommand => Some(Cmd::new()),
            RequestType::Get => Some(cmd("GET")),
            RequestType::Set => Some(cmd("SET")),
            RequestType::Ping => Some(cmd("PING")),
            RequestType::Info => Some(cmd("INFO")),
            RequestType::Del => Some(cmd("DEL")),
            RequestType::Select => Some(cmd("SELECT")),
            RequestType::ConfigGet => Some(get_two_word_command("CONFIG", "GET")),
            RequestType::ConfigSet => Some(get_two_word_command("CONFIG", "SET")),
            RequestType::ConfigResetStat => Some(get_two_word_command("CONFIG", "RESETSTAT")),
            RequestType::ConfigRewrite => Some(get_two_word_command("CONFIG", "REWRITE")),
            RequestType::ClientGetName => Some(get_two_word_command("CLIENT", "GETNAME")),
            RequestType::ClientGetRedir => Some(get_two_word_command("CLIENT", "GETREDIR")),
            RequestType::ClientId => Some(get_two_word_command("CLIENT", "ID")),
            RequestType::ClientInfo => Some(get_two_word_command("CLIENT", "INFO")),
            RequestType::ClientKill => Some(get_two_word_command("CLIENT", "KILL")),
            RequestType::ClientList => Some(get_two_word_command("CLIENT", "LIST")),
            RequestType::ClientNoEvict => Some(get_two_word_command("CLIENT", "NO-EVICT")),
            RequestType::ClientNoTouch => Some(get_two_word_command("CLIENT", "NO-TOUCH")),
            RequestType::ClientPause => Some(get_two_word_command("CLIENT", "PAUSE")),
            RequestType::ClientReply => Some(get_two_word_command("CLIENT", "REPLY")),
            RequestType::ClientSetInfo => Some(get_two_word_command("CLIENT", "SETINFO")),
            RequestType::ClientSetName => Some(get_two_word_command("CLIENT", "SETNAME")),
            RequestType::ClientUnblock => Some(get_two_word_command("CLIENT", "UNBLOCK")),
            RequestType::ClientUnpause => Some(get_two_word_command("CLIENT", "UNPAUSE")),
            RequestType::Expire => Some(cmd("EXPIRE")),
            RequestType::HSet => Some(cmd("HSET")),
            RequestType::HGet => Some(cmd("HGET")),
            RequestType::HDel => Some(cmd("HDEL")),
            RequestType::HExists => Some(cmd("HEXISTS")),
            RequestType::MSet => Some(cmd("MSET")),
            RequestType::MGet => Some(cmd("MGET")),
            RequestType::Incr => Some(cmd("INCR")),
            RequestType::IncrBy => Some(cmd("INCRBY")),
            RequestType::IncrByFloat => Some(cmd("INCRBYFLOAT")),
            RequestType::Decr => Some(cmd("DECR")),
            RequestType::DecrBy => Some(cmd("DECRBY")),
            RequestType::HGetAll => Some(cmd("HGETALL")),
            RequestType::HMSet => Some(cmd("HMSET")),
            RequestType::HMGet => Some(cmd("HMGET")),
            RequestType::HIncrBy => Some(cmd("HINCRBY")),
            RequestType::HIncrByFloat => Some(cmd("HINCRBYFLOAT")),
            RequestType::LPush => Some(cmd("LPUSH")),
            RequestType::LPop => Some(cmd("LPOP")),
            RequestType::RPush => Some(cmd("RPUSH")),
            RequestType::RPop => Some(cmd("RPOP")),
            RequestType::LLen => Some(cmd("LLEN")),
            RequestType::LRem => Some(cmd("LREM")),
            RequestType::LRange => Some(cmd("LRANGE")),
            RequestType::LTrim => Some(cmd("LTRIM")),
            RequestType::SAdd => Some(cmd("SADD")),
            RequestType::SRem => Some(cmd("SREM")),
            RequestType::SMembers => Some(cmd("SMEMBERS")),
            RequestType::SCard => Some(cmd("SCARD")),
            RequestType::PExpireAt => Some(cmd("PEXPIREAT")),
            RequestType::PExpire => Some(cmd("PEXPIRE")),
            RequestType::ExpireAt => Some(cmd("EXPIREAT")),
            RequestType::Exists => Some(cmd("EXISTS")),
            RequestType::Unlink => Some(cmd("UNLINK")),
            RequestType::TTL => Some(cmd("TTL")),
            RequestType::ZAdd => Some(cmd("ZADD")),
            RequestType::ZRem => Some(cmd("ZREM")),
            RequestType::ZRange => Some(cmd("ZRANGE")),
            RequestType::ZCard => Some(cmd("ZCARD")),
            RequestType::ZCount => Some(cmd("ZCOUNT")),
            RequestType::ZIncrBy => Some(cmd("ZINCRBY")),
            RequestType::ZScore => Some(cmd("ZSCORE")),
            RequestType::Type => Some(cmd("TYPE")),
            RequestType::HLen => Some(cmd("HLEN")),
            RequestType::Echo => Some(cmd("ECHO")),
            RequestType::ZPopMin => Some(cmd("ZPOPMIN")),
            RequestType::Strlen => Some(cmd("STRLEN")),
            RequestType::LIndex => Some(cmd("LINDEX")),
            RequestType::ZPopMax => Some(cmd("ZPOPMAX")),
            RequestType::XAck => Some(cmd("XACK")),
            RequestType::XAdd => Some(cmd("XADD")),
            RequestType::XReadGroup => Some(cmd("XREADGROUP")),
            RequestType::XRead => Some(cmd("XREAD")),
            RequestType::XGroupCreate => Some(get_two_word_command("XGROUP", "CREATE")),
            RequestType::XGroupDestroy => Some(get_two_word_command("XGROUP", "DESTROY")),
            RequestType::XTrim => Some(cmd("XTRIM")),
            RequestType::HSetNX => Some(cmd("HSETNX")),
            RequestType::SIsMember => Some(cmd("SISMEMBER")),
            RequestType::HVals => Some(cmd("HVALS")),
            RequestType::PTTL => Some(cmd("PTTL")),
            RequestType::ZRemRangeByRank => Some(cmd("ZREMRANGEBYRANK")),
            RequestType::Persist => Some(cmd("PERSIST")),
            RequestType::ZRemRangeByScore => Some(cmd("ZREMRANGEBYSCORE")),
            RequestType::Time => Some(cmd("TIME")),
            RequestType::ZRank => Some(cmd("ZRANK")),
            RequestType::Rename => Some(cmd("RENAME")),
            RequestType::DBSize => Some(cmd("DBSIZE")),
            RequestType::BRPop => Some(cmd("BRPOP")),
            RequestType::HKeys => Some(cmd("HKEYS")),
            RequestType::PfAdd => Some(cmd("PFADD")),
            RequestType::PfCount => Some(cmd("PFCOUNT")),
            RequestType::PfMerge => Some(cmd("PFMERGE")),
            RequestType::RPushX => Some(cmd("RPUSHX")),
            RequestType::LPushX => Some(cmd("LPUSHX")),
            RequestType::BLPop => Some(cmd("BLPOP")),
            RequestType::LInsert => Some(cmd("LINSERT")),
            RequestType::SPop => Some(cmd("SPOP")),
            RequestType::ZMScore => Some(cmd("ZMSCORE")),
            RequestType::ZDiff => Some(cmd("ZDIFF")),
            RequestType::ZDiffStore => Some(cmd("ZDIFFSTORE")),
            RequestType::SetRange => Some(cmd("SETRANGE")),
            RequestType::ZRemRangeByLex => Some(cmd("ZREMRANGEBYLEX")),
            RequestType::ZLexCount => Some(cmd("ZLEXCOUNT")),
            RequestType::Append => Some(cmd("APPEND")),
            RequestType::SDiffStore => Some(cmd("SDIFFSTORE")),
            RequestType::SInter => Some(cmd("SINTER")),
            RequestType::SInterStore => Some(cmd("SINTERSTORE")),
            RequestType::SUnionStore => Some(cmd("SUNIONSTORE")),
            RequestType::ZRangeStore => Some(cmd("ZRANGESTORE")),
            RequestType::GetRange => Some(cmd("GETRANGE")),
            RequestType::SMove => Some(cmd("SMOVE")),
            RequestType::SMIsMember => Some(cmd("SMISMEMBER")),
            RequestType::ZUnionStore => Some(cmd("ZUNIONSTORE")),
            RequestType::LastSave => Some(cmd("LASTSAVE")),
            RequestType::GeoAdd => Some(cmd("GEOADD")),
            RequestType::GeoHash => Some(cmd("GEOHASH")),
            RequestType::ObjectEncoding => Some(get_two_word_command("OBJECT", "ENCODING")),
            RequestType::ObjectFreq => Some(get_two_word_command("OBJECT", "FREQ")),
            RequestType::ObjectIdleTime => Some(get_two_word_command("OBJECT", "IDLETIME")),
            RequestType::GeoDist => Some(cmd("GEODIST")),
            RequestType::SDiff => Some(cmd("SDIFF")),
            RequestType::ObjectRefCount => Some(get_two_word_command("OBJECT", "REFCOUNT")),
            RequestType::Lolwut => Some(cmd("LOLWUT")),
            RequestType::GeoPos => Some(cmd("GEOPOS")),
            RequestType::BZPopMax => Some(cmd("BZPOPMAX")),
            RequestType::RenameNX => Some(cmd("RENAMENX")),
            RequestType::Touch => Some(cmd("TOUCH")),
            RequestType::ZRevRank => Some(cmd("ZREVRANK")),
            RequestType::ZInterStore => Some(cmd("ZINTERSTORE")),
            RequestType::HRandField => Some(cmd("HRANDFIELD")),
            RequestType::ZUnion => Some(cmd("ZUNION")),
            RequestType::BZPopMin => Some(cmd("BZPOPMIN")),
            RequestType::FlushAll => Some(cmd("FLUSHALL")),
            RequestType::ZRandMember => Some(cmd("ZRANDMEMBER")),
            RequestType::Bitcount => Some(cmd("BITCOUNT")),
            RequestType::BZMPop => Some(cmd("BZMPOP")),
            RequestType::ZInterCard => Some(cmd("ZINTERCARD")),
            RequestType::ZMPop => Some(cmd("ZMPOP")),
            RequestType::FunctionLoad => Some(get_two_word_command("FUNCTION", "LOAD")),
        }
    }
}
