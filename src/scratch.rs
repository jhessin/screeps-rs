
/// TODO Refactor the creeper implementation
/// Creeper should only have a few worker methods
/// - new() works fine - but may parse extra data for easier retrieval
///   * Possibly use names_for_role() to populate a count of each role.
/// ** Utility methods
/// - names_for_role() - seems to be working well.
/// - run() of course should remain the same
/// - is_working() also should remain the same except:
///   * Should clear source/target when changing.
/// - data() also remains if it isn't folded into the data model.
/// - save() - may need a renew if modifying data model
/// - handle_code() - TODO: rename to make more sense possibly: move_or()

/// ** Gather methods:
///   * These should all check to ensure is_working() returns false
///   * Should check for existing target for better performance.
/// 
/// -- harvest() should be multi-purpose for harvesting/mining from sources
///   * This should also gather from dropped resources, containers, Tombstones, and Structures
/// 
/// -- gather_storage() should gather resources for storage
/// 
/// -- gather_work() should gather resources for work from containers, dropped, resources, etc.
///   * This can also pull from Storage, Links, etc.

/// ** Deliver methods:
///   * These should all check to ensure is_working() returns true
/// -- deliver() should deliver resources by priority for storage/use.
/// -- build() should build any construction sites.
/// -- repair() should repair the nearest damaged structure
/// -- repair_wall() should repair the appropriate wall
/// -- upgrade() should upgrade tho room controller
