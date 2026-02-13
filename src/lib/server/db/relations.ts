import { defineRelations } from 'drizzle-orm';
import { users, follows } from './schema';

export const relations = defineRelations({ users, follows }, (r) => ({
    users: {
        following: r.many.users({
            from: r.users.id.through(r.follows.followerId),
            to: r.users.id.through(r.follows.followingId),
        }),
        followers: r.many.users({
            from: r.users.id.through(r.follows.followingId),
            to: r.users.id.through(r.follows.followerId),
        })
    }
}));
