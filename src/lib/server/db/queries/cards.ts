import { drizzle } from "drizzle-orm/d1";
// import type { CachedCard } from "../types";
import * as schema from '../../db/schema';
import { eq, and, sql, asc, desc, inArray } from 'drizzle-orm';

// todo