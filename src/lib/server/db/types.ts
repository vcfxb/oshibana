// types from the DB

import * as schema from './schema';

export type DbUser = typeof schema.users.$inferSelect;
export type DbSession = typeof schema.sessions.$inferSelect;
export type DbCachedCard = typeof schema.cardCache.$inferSelect;
export type DbStorageLocation = typeof schema.storageLocations.$inferSelect;


