/**
 * Pull bulk scryfall data and load it into the DB's cards table.
 * 
 */

import { parseArgs } from "util";
import Cloudflare from 'cloudflare';

function main() {
    const { values: { target } } = parseArgs({
        options: {
            target: {
                type: 'string',
                short: 't',
                default: 'local'
            }
        }
    });

    if (!['local', 'prod'].includes(target.toLowerCase())) {
        console.error("target must be either local or prod");
        process.exitCode = 1;
        return;
    }




}

main();
