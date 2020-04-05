
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
/// 
/// ** Finder methods - TODO: attach to a trait for Vec<Target> or expand the existing Finder struct.
///   * These should make it easier to filter/build Vec<Target>'s
///   * They will all return Targets.
///
/// - find_nearest_work_energy()
///   * This is a super method that gets the closest energy source
///   * excluding spawns/extensions/sources
///   * includes lorries that are working.
///
/// - find_nearest_store_energy()
///   * This is a super method that gets the closest energy source for storage
///   * doesn't include buildings other than Ruins, Tombstones, and Containers
///
/// - find_nearest_active_source()
///   * Strictly for HARVESTERS
///
/// - find_nearest_dropped_resource()
/// - find_nearest_tombstone()
/// - find_nearest_ruin()
/// - find_nearest_other_energy_source()
///   * Should return the nearest of any structure with stored energy that is not a
///     * spawn/extension/Tower that has stored energy
///   * May be refactored to exclude others when they are unlocked.
///
/// - find_nearest_storage()
///   * This returns
/// - find_nearest_tower_needing_energy()
/// - find_nearest_spawn_extension_needing_energy()
/// - find nearest_other_energy_target()
///   * Should return the nearest of any structure with room for more energy.
/// - find_nearest_construction_site()
/// - find_nearest_repair_target()
/// - find_nearest_wall_repair_target()
///
///
///
///
///
/// ** Gather methods:
///   * These should all check to ensure is_working() returns false
///   * Should check for existing target for better performance.
/// -- harvest() should be multi-purpose for harvesting/mining from sources
///   * This should also gather from dropped resources, containers, Tombstones, and Structures
/// -- gather_storage() should gather resources for storage
/// -- gather_work() should gather resources for work from containers, dropped, resources, etc.
///   * This can also pull from Storage, Links, etc.
///
/// ** Deliver methods:
///   * These should all check to ensure is_working() returns true
/// -- deliver() should deliver resources by priority for storage/use.
/// -- build() should build any construction sites.
/// -- repair() should repair the nearest damaged structure
/// --
