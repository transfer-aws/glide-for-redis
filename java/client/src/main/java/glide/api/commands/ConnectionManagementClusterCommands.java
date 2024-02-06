/** Copyright GLIDE-for-Redis Project Contributors - SPDX Identifier: Apache-2.0 */
package glide.api.commands;

import glide.api.models.configuration.RequestRoutingConfiguration.Route;
import java.util.concurrent.CompletableFuture;

/**
 * Connection Management Commands interface.
 *
 * @see: <a href="https://redis.io/commands/?group=connection">Connection Management Commands</a>
 */
public interface ConnectionManagementClusterCommands {

    /**
     * Ping the Redis server.
     *
     * @param route Routing configuration for the command
     * @see <a href="https://redis.io/commands/ping/">redis.io</a> for details.
     * @return Response from Redis containing a <code>String</code>.
     */
    CompletableFuture<String> ping(Route route);

    /**
     * Ping the Redis server.
     *
     * @see <a href="https://redis.io/commands/ping/">redis.io</a> for details.
     * @param msg The ping argument that will be returned.
     * @param route Routing configuration for the command
     * @return Response from Redis containing a <code>String</code>.
     */
    CompletableFuture<String> ping(String msg, Route route);
}
