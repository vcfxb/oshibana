import { defineRelations } from 'drizzle-orm';
import { users, follows, storageLocations, physicalCards, cardCache } from './schema';

const tables = { users, follows, storageLocations, physicalCards, cardCache };

export const relations = defineRelations(tables, (r) => ({
    users: {
        following: r.many.users({
            from: r.users.id.through(r.follows.followerId),
            to: r.users.id.through(r.follows.followingId),
        }),
        followers: r.many.users({
            from: r.users.id.through(r.follows.followingId),
            to: r.users.id.through(r.follows.followerId),
        }),
        storage_locations: r.many.storageLocations({
            from: r.users.id,
            to: r.storageLocations.userId,
        })
    },

    storageLocations: {
        owner: r.one.users({
            from: r.storageLocations.userId,
            to: r.users.id,
            optional: false,
        })
    },

    physicalCards: {
        card: r.one.cardCache({
            from: r.physicalCards.scryfallId,
            to: r.cardCache.scryfallId,
            optional: false,
        })
    }
}));
