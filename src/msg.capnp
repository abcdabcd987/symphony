@0xe2783e4a08c80e7c;

###### Commons

using NodeId = UInt64;
using QueryId = UInt64;
using PlanId = UInt64;
using TimestampNs = Int64;

struct ModelSession {
    framework @0 :Text;
    model @1 :Text;
    sloMs @2 :UInt32;
}

struct EncodedImage {
    enum Encoding {
        jpg @0;
        png @1;
    }

    encoding @0 :Encoding;
    bytes @1 :Data;
}

struct QueryPunchClock {
    frontendId @0 :NodeId;
    frontendRecvTime @1 :TimestampNs;
    scheduerId @2 :NodeId;
    schedulerRecvTime @3 :TimestampNs;
    schedulerSendTime @4 :TimestampNs;
    backendId @5 :NodeId;
    backendRecvTime @6 :TimestampNs;
    backendGetImageTime @7 :TimestampNs;
    backendPreprocDoneTime @8 :TimestampNs;
    backendBatchStartTime @9 :TimestampNs;
}

###### RPC servers

struct BackendMessage {
    union {
        load @0 :LoadModelSessionCommand;
        query @1 :AssignQueryCommand;
        images @2 :ReadImagesRpcReply;
        plan @3 :AssignPlanCommand;
    }
}

struct FrontendMessage {
    read @0 :ReadImageRpcRequest;
}

###### RPC messages

# Scheduler -> Backend
struct LoadModelSessionCommand {
    modelSession @0 :ModelSession;
}

# Scheduler -> Backend
struct AssignQueryCommand {
    queryId @0 :QueryId;
    modelSession @1 :ModelSession;
    clock @2 :QueryPunchClock;
}

# Scheduler -> Backend
struct AssignPlanCommand {
    planId @0 :PlanId;
    modelSession @1 :ModelSession;
    execTime @2 :TimestampNs;
    queryIds @3 :List(QueryId);

    expFinishTime @4 :TimestampNs;
}

# Backend -> Frontend
struct ReadImageRpcRequest {
    queryIds @0 :List(QueryId);
}

# Frontend -> Backend
struct ReadImagesRpcReply {
    struct QueryImage {
        queryId @0 :QueryId;
        image @1 :EncodedImage;
    }

    images @0 :List(QueryImage);
}
