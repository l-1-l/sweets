use std::{
    // error::Error,
    fmt,
    num::{NonZeroU16, NonZeroU32},
};

/// A possible error value when converting a `StatusCode` from a `u16` or `&str`
///
/// This error indicates that the supplied input was not a valid number, was less
/// than 1000, or was greater than 6900.
// pub struct InvalidBussinessStatusCode {
//     _priv: (),
// }

// impl InvalidBussinessStatusCode {
//     fn new() -> InvalidBussinessStatusCode {
//         InvalidBussinessStatusCode { _priv: () }
//     }
// }

// impl fmt::Debug for InvalidBussinessStatusCode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("InvalidBussinessStatusCode").finish()
//     }
// }

// impl fmt::Display for InvalidBussinessStatusCode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.write_str("Invalid bussiness status code")
//     }
// }

// impl Error for InvalidBussinessStatusCode {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResponseStatus(
    /// Business status code
    NonZeroU32,
    /// reason
    &'static str,
    /// Http status code  
    NonZeroU16,
);

impl Default for ResponseStatus {
    #[inline]
    fn default() -> Self {
        ResponseStatus::OK
    }
}

impl fmt::Debug for ResponseStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // fmt::Debug::fmt(&self, f)
        f.debug_tuple("ResponseStatus")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

impl PartialEq<u32> for ResponseStatus {
    #[inline]
    fn eq(&self, other: &u32) -> bool {
        self.bussiness_code_as_u32() == *other
    }
}

impl PartialEq<ResponseStatus> for u32 {
    fn eq(&self, other: &ResponseStatus) -> bool {
        *self == other.bussiness_code_as_u32()
    }
}

impl ResponseStatus {
    #[inline]
    pub fn http_status_code_as_u16(&self) -> u16 {
        self.2.into()
    }

    #[inline]
    pub fn bussiness_code_as_u32(&self) -> u32 {
        self.0.into()
    }

    pub fn reason(&self) -> &'static str {
        self.1
    }

    /// Check  if http status code is 2xx
    #[inline]
    pub fn is_success(&self) -> bool {
        300 > self.2.get() && self.2.get() >= 200
    }

    /// Check  if http status code is 4xx
    #[inline]
    pub fn is_client_error(&self) -> bool {
        500 > self.2.get() && self.2.get() >= 400
    }

    /// Check if status code is 5xx
    #[inline]
    pub fn is_server_error(&self) -> bool {
        600 > self.2.get() && self.2.get() >= 500
    }
}

macro_rules! response_status{
    (
        $(
            // $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr, $http_status:expr);
        )+
    ) => {
        impl ResponseStatus {
        $(
            // $(#[$docs])*
            pub const $konst: ResponseStatus = ResponseStatus(unsafe { NonZeroU32::new_unchecked($num) }, $phrase, unsafe { NonZeroU16::new_unchecked($http_status) });
        )+

        }
    }
}

response_status!(
    // Successful responses

    (1000, OK, "ok", 200);
    (1001, NO_CONTENT, "No content", 204);
    (1002, ALREADY_UP_TO_DATE, "Already up-to-date", 204);

    // **********************************************************
    // * For application error
    // **********************************************************

    // Client

    (1100, INVALID_REQUEST, "The client request is invalid", 400);
    (1101, CLIENT_REQUEST_TOO_FREQUENT, "The client request are too frequent", 429);
    (1102, ILLEGAL_ARGUMENT, "Illegal argument", 400);
    (1103, RECORD_CONTAINS_DUPLICATE_KEY, "The record to add contains a duplicate key", 409);
    (1104, REQUESTED_RECORDS_TOO_MANY, "Too many records are requested", 429);
    (1105, SEND_REQUEST_FROM_NON_EXISING_SESSION, "The session should be established before sending requests", 403);

    // Server

    (1200, SERVER_INTERNAL_ERROR, "Internal server error", 500);
    (1201, SERVER_UNAVAILABLE, "The server is unavailable", 503);

    // **********************************************************
    // * For error about admin activity
    // **********************************************************

    // Admin

    (1300, UNAUTHORIZED, "Unauthorized", 401);
    (1301, NO_FILTER_FOR_DELETE_OPERATION, "Delete operation should have at least one filter", 400);
    (1302, RESOURCE_NOT_FOUND, "Resource not found", 404);
    (1310, DUMP_JFR_IN_ILLEGAL_STATUS, "Dumping JFR  should be executed in a legal status", 406);
    (1311, ADMIN_REQUESTS_TOO_FREQUENT, "Admin requests are too frequent", 429);


    // Blocklist

    (1400, IP_BLOCKLIST_IS_DISABLED, "Blocking an IP is disbaled", 403);
    (1401, USER_ID_BLOCKLIST_IS_DISABLED, "Blocking a user id ID disbaled", 403);

    // Cluster - Leader

    (1800, NON_EXISTING_MEMBER_TO_BE_LEADER, "Cannot find the node", 404);
    (1801, NO_QUALIFIED_MEMBER_TO_BE_LEADER, "No qualified node to be a leader", 503);
    (1802, NOT_QUALIFIED_MEMBER_TO_BE_LEADER, "Only qualified node (isLeaderEligible=true, nodeType=SERVICE) can be a leader", 403);


    // **********************************************************
    // * For business error
    // **********************************************************

    // User

    // User - Login
    (2000, UNSUPPORTED_CLIENT_VERSION, "The version of the client isn't supported", 403);

    (2010, LOGIN_TIMEOUT, "Login timeout", 408);
    (2011, LOGIN_AUTHENTICATION_FAILED, "The user's login details are incorrect", 401);
    (2012, LOGGING_IN_USER_NOT_ACTIVE, "The logging in user is inactive", 401);
    (2013, LOGIN_FROM_FORBIDDEN_DEVICE_TYPE, "The device type is forbidden to login", 401);

    // User -Session
    (2100,SESSION_SIMULTANEOUS_CONFLICTS_DECLINE, "A different device has logged into your account", 409);
    (2101,SESSION_SIMULTANEOUS_CONFLICTS_NOTIFY, "A different device attempted to log into your account", 409);
    (2102,SESSION_SIMULTANEOUS_CONFLICTS_OFFLINE, "A different device has logged into your account", 409);
    (2103 , CREATE_EXISTING_SESSION, "The session has existed", 503);
    (2104, UPDATE_NON_EXISTING_SESSION_HEARTBEAT, "Cannot update the heartbeat of a non-existing session", 403);



    // User - Location
    (2200, USER_LOCATION_RELATED_FEATURES_ARE_DISABLED,"The features related to user location are disabled", 510);
    (2201,QUERYING_NEAREST_USERS_BY_SESSION_ID_IS_DISABLED,
            "The feature to query nearest users by session IDs is disabled", 510);

    // User - Info
    (2300, UPDATE_INFO_OF_NON_EXISTING_USER,"Cannot update a non-existing user's information", 403);
    (2301, USER_PROFILE_NOT_FOUND,"User profile not found", 404);
    (2302,PROFILE_REQUESTER_NOT_IN_CONTACTS_OR_BLOCKED, "The profile requester isn't in contacts or is blocked", 403);
    (2303, PROFILE_REQUESTER_HAS_BEEN_BLOCKED,"The profile requester has been blocked", 403);

    // User - Permission
    (2400, QUERY_PERMISSION_OF_NON_EXISTING_USER,"Cannot query a non-existing user's permission", 404);

    // User - Relationship
    (2500, ADD_NOT_RELATED_USER_TO_GROUP,"Cannot add a not related user to a relationship group", 403);
    (2501, CREATE_EXISTING_RELATIONSHIP, "Cannot create an existing relationship", 409);

    // User - Friend Request
    (2600,REQUESTER_NOT_FRIEND_REQUEST_RECIPIENT,
            "Only the recipient of the friend request can handle the friend request", 403);
    (2601, CREATE_EXISTING_FRIEND_REQUEST,"A friend request has already existed", 409);
    (2602,FRIEND_REQUEST_SENDER_HAS_BEEN_BLOCKED, "The friend request sender has been blocked by the recipient", 403);

    // Group

    // Group - Info

    (3000,UPDATE_INFO_OF_NON_EXISTING_GROUP, "Cannot update the information of a non-existing group", 403);
    (3001, NOT_OWNER_TO_UPDATE_GROUP_INFO,"Only the group owner can update the group information", 403);
    (3002,
        NOT_OWNER_OR_MANAGER_TO_UPDATE_GROUP_INFO,
            "Only the owner and managers of the group can update the group information", 403);
    (3003, NOT_MEMBER_TO_UPDATE_GROUP_INFO,"Only the group members can update the group information", 403);

    // Group - Type

    (3100,NO_PERMISSION_TO_CREATE_GROUP_WITH_GROUP_TYPE, "No permission to create a group with the group type", 403);
    (3101,CREATE_GROUP_WITH_NON_EXISTING_GROUP_TYPE, "Cannot create a group with a non-existing group type", 403);

    // Group - Ownership

    (3200,NOT_ACTIVE_USER_TO_CREATE_GROUP, "The user trying to create a group is inactive", 403);
    (3201,NOT_OWNER_TO_TRANSFER_GROUP, "Only the group owner can transfer the group", 403);
    (3202,NOT_OWNER_TO_DELETE_GROUP, "Only the group owner can delete the group", 403);
    (3203, SUCCESSOR_NOT_GROUP_MEMBER,"The successor must be a member of the group", 403);

  (3204,OWNER_QUITS_WITHOUT_SPECIFYING_SUCCESSOR, "The successor ID must be specified when the owner quits the group",
  400);

  (3205,MAX_OWNED_GROUPS_REACHED, "The user has reached the maximum allowed owned groups", 403);

  (3206,TRANSFER_NON_EXISTING_GROUP, "Cannot transfer a non-existing group", 403);

  // Group - Question

  (3300,NOT_OWNER_OR_MANAGER_TO_CREATE_GROUP_QUESTION,
  "Only the owner and managers of the group can create group questions", 403);
  (3301,NOT_OWNER_OR_MANAGER_TO_DELETE_GROUP_QUESTION,
  "Only the owner and managers of the group can delete group questions", 403);
  (3302,NOT_OWNER_OR_MANAGER_TO_UPDATE_GROUP_QUESTION,
  "Only the owner and managers of the group can update group questions", 403);
  (3303,NOT_OWNER_OR_MANAGER_TO_ACCESS_GROUP_QUESTION_ANSWER,
  "Only the owner and managers of the group can access group question answers",
  403);
  (3304,GROUP_QUESTION_ANSWERER_HAS_BEEN_BLOCKED, "The group question answerer has been blocked", 403);
  (3305,MEMBER_CANNOT_ANSWER_GROUP_QUESTION, "A group member cannot answer group questions", 409);
  (3306,ANSWER_QUESTION_OF_INACTIVE_GROUP, "Cannot answer the questions of an inactive group", 403);


  // Group - Member

  (3400,NOT_OWNER_OR_MANAGER_TO_REMOVE_GROUP_MEMBER,
  "Only the owner and managers of the group can remove the group member", 403);
  (3401,NOT_OWNER_TO_UPDATE_GROUP_MEMBER_INFO, "Only the group owner can update the group member's information", 403);
  (3402,NOT_OWNER_OR_MANAGER_TO_UPDATE_GROUP_MEMBER_INFO,
  "Only the owner and managers of the group can update the group member's information", 403);
  (3403,NOT_MEMBER_TO_QUERY_MEMBER_INFO, "Only the group members can query its group members' information", 403);
  (3404,ADD_BLOCKED_USER_TO_GROUP, "Cannot add a blocked user to the group", 403);
  (3405,ADD_BLOCKED_USER_TO_INACTIVE_GROUP, "Cannot add a blocked user to the inactive group", 403);
  (3406,ADD_USER_TO_INACTIVE_GROUP, "Cannot add a user to the inactive group", 403);
  (3407,ADD_NEW_MEMBER_WITH_ROLE_HIGHER_THAN_REQUESTER, "Cannot add a user with the role higher than the requester's",
  403);

  // Group - Blocklist

  (3500,NOT_OWNER_OR_MANAGER_TO_ADD_BLOCKED_USER, "Only the owner and managers of the group can add blocked users",
  403);
  (3501,NOT_OWNER_OR_MANAGER_TO_REMOVE_BLOCKED_USER,
  "Only the owner and managers of the group can remove blocked users", 403);

  // Group - Join Request

  (3600,GROUP_JOIN_REQUEST_SENDER_HAS_BEEN_BLOCKED, "The group join request sender has been blocked", 403);
  (3601,NOT_JOIN_REQUEST_SENDER_TO_RECALL_REQUEST, "Only the join request sender can recall the request", 403);
  (3602,NOT_OWNER_OR_MANAGER_TO_ACCESS_GROUP_REQUEST,
  "Only the owner and managers of the group can access group requests", 403);
  (3603,RECALL_NOT_PENDING_GROUP_JOIN_REQUEST, "Cannot recall not pending group join requests", 403);
  (3604,SEND_JOIN_REQUEST_TO_INACTIVE_GROUP, "Cannot send a join request to an inactive group", 403);
  (3605,RECALLING_GROUP_JOIN_REQUEST_IS_DISABLED, "The feature to recall group join requests is disabled", 510);

  // Group - Invitation

  (3700,GROUP_INVITER_NOT_MEMBER, "Only the group members can invite other users", 403);
  (3701,GROUP_INVITEE_ALREADY_GROUP_MEMBER, "The invitee is already a member of the group", 409);
  (3702,NOT_OWNER_OR_MANAGER_TO_RECALL_INVITATION, "Only the owner and managers of the group can recall invitations",
  403);
  (3703,NOT_OWNER_OR_MANAGER_TO_ACCESS_INVITATION, "Only the owner and managers of the group can access invitations",
  403);
  (3704,NOT_OWNER_TO_SEND_INVITATION, "Only the group owner can send invitations", 403);
  (3705,NOT_OWNER_OR_MANAGER_TO_SEND_INVITATION, "Only the owner and managers can send invitations", 403);
  (3706,NOT_MEMBER_TO_SEND_INVITATION, "Only the group members can send invitations", 403);
  (3707,INVITEE_HAS_BEEN_BLOCKED, "The invitee has been blocked by the group", 403);
  (3708,RECALLING_GROUP_INVITATION_IS_DISABLED, "The feature to recall group invitations is disabled", 510);
  (3709,REDUNDANT_GROUP_INVITATION, "The group invitation is redundant", 406);
  (3710,RECALL_NOT_PENDING_GROUP_INVITATION, "Cannot recall not pending group invitations", 403);

  // Conversation

  (4000,UPDATING_TYPING_STATUS_IS_DISABLED, "The feature to update the typing status is disabled", 510);
  (4001,UPDATING_READ_DATE_IS_DISABLED, "The feature to update the read data is disabled", 510);
  (4002,MOVING_READ_DATE_FORWARD_IS_DISABLED, "The feature to move the read data forward is disabled", 510);

  // Message
  // Message - Send

  (5000,MESSAGE_RECIPIENT_NOT_ACTIVE, "The message recipient is inactive", 403);
  (5001,MESSAGE_SENDER_NOT_IN_CONTACTS_OR_BLOCKED,
  "The message sender isn't in contacts or is blocked by the recipient", 403);
  (5002,PRIVATE_MESSAGE_SENDER_HAS_BEEN_BLOCKED, "The private message sender has been blocked", 403);
  (5003,GROUP_MESSAGE_SENDER_HAS_BEEN_BLOCKED, "The group message sender has been blocked", 403);
  (5004,SEND_MESSAGE_TO_INACTIVE_GROUP, "Cannot send a message to an inactive group", 403);
  (5005,SEND_MESSAGE_TO_MUTED_GROUP, "Cannot send a message to a muted group", 403);
  (5006,SENDING_MESSAGES_TO_ONESELF_IS_DISABLED, "The feature to send a message to oneself is disabled", 510);
  (5007,MUTED_MEMBER_SEND_MESSAGE, "The muted group member cannot send a message", 403);
  (5008,GUESTS_HAVE_BEEN_MUTED, "All guests of the group have been muted", 403);
  (5009,MESSAGE_IS_ILLEGAL, "The message contains unwanted words", 403);

  // Message - Update

  (5100,UPDATING_MESSAGE_BY_SENDER_IS_DISABLED, "The feature to update messages sent by the sender is disabled", 510);
  (5101,NOT_SENDER_TO_UPDATE_MESSAGE, "Only the message sender can update the message", 403);

  // TODO: remove?

  (5102,NOT_MESSAGE_RECIPIENT_TO_UPDATE_MESSAGE_READ_DATE, "Only the message recipient can update the read date", 403);

  // Message - Recall

  (5200,RECALL_NON_EXISTING_MESSAGE, "Cannot recall a non-existing message", 403);
  (5201,RECALLING_MESSAGE_IS_DISABLED, "The feature to recall messages is disabled", 510);
  (5202,MESSAGE_RECALL_TIMEOUT, "The maximum allowed time to recall the message has passed", 403);

  // Storage

  (6000,STORAGE_NOT_IMPLEMENTED, "The storage feature is enabled but not implemented yet", 501);
  (6001,FILE_TOO_LARGE, "The file is too large to upload", 413);

  // Storage - Extension

  (6900,REDUNDANT_REQUEST_FOR_PRESIGNED_PROFILE_URL, "The request for the presigned profile URL is redundant", 406);
);

fn d() {
    let c = ResponseStatus::OK;
    // ResponseStatus::cano
}
