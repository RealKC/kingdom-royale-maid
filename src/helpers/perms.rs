use serenity::model::{
    channel::{PermissionOverwrite, PermissionOverwriteType},
    prelude::*,
};

pub fn make_allowed_override_for_user(user: UserId) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::READ_MESSAGES
            | Permissions::SEND_MESSAGES
            | Permissions::READ_MESSAGE_HISTORY,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Member(user),
    }
}

pub fn make_denied_override_for_user(user: UserId) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::READ_MESSAGES
            | Permissions::SEND_MESSAGES
            | Permissions::READ_MESSAGE_HISTORY,
        kind: PermissionOverwriteType::Member(user),
    }
}

pub fn make_allowed_override_for_role(role: RoleId) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::SEND_MESSAGES
            | Permissions::READ_MESSAGES
            | Permissions::READ_MESSAGE_HISTORY,
        deny: Permissions::empty(),
        kind: PermissionOverwriteType::Role(role),
    }
}

pub fn make_denied_override_for_role(role: RoleId) -> PermissionOverwrite {
    PermissionOverwrite {
        allow: Permissions::empty(),
        deny: Permissions::READ_MESSAGES
            | Permissions::SEND_MESSAGES
            | Permissions::READ_MESSAGE_HISTORY,
        kind: PermissionOverwriteType::Role(role),
    }
}
