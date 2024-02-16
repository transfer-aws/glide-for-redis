/** Copyright GLIDE-for-Redis Project Contributors - SPDX Identifier: Apache-2.0 */
package glide.api.commands;

import java.util.concurrent.CompletableFuture;

/**
 * Generic Commands interface to handle generic commands for all server requests for both standalone
 * and cluster clients.
 *
 * @see <a href="https://redis.io/commands/?group=generic">Generic Commands</a>
 */
public interface GenericBaseCommands {

    /**
     * Removes the specified <code>keys</code> from the database. A key is ignored if it does not
     * exist.
     *
     * @see <a href="https://redis.io/commands/del/">redis.io</a> for details.
     * @param keys The keys we wanted to remove.
     * @return The number of keys that were removed.
     */
    CompletableFuture<Long> del(String[] keys);

    /**
     * Returns the number of keys in <code>keys</code> that exist in the database.
     *
     * @see <a href="https://redis.io/commands/exists/">redis.io</a> for details.
     * @param keys The keys list to check.
     * @return The number of keys that exist. If the same existing key is mentioned in <code>keys
     *     </code> multiple times, it will be counted multiple times.
     */
    CompletableFuture<Long> exists(String[] keys);
}
