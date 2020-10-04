mod muteall;
mod unmuteall;

use serenity::framework::standard::macros::group;
use muteall::*;
use unmuteall::*;

#[group]
#[commands(muteall, unmuteall)]
struct Muting;
