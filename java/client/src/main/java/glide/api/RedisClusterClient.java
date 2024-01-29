package glide.api;

import glide.api.commands.ClusterBaseCommands;
import glide.api.models.ClusterValue;
import glide.api.models.configuration.RedisClusterClientConfiguration;
import glide.api.models.configuration.RequestRoutingConfiguration.Route;
import glide.connectors.handlers.ChannelHandler;
import glide.managers.CommandManager;
import glide.managers.ConnectionManager;
import glide.managers.models.Command;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

/**
 * Async (non-blocking) client for Redis in Cluster mode. Use {@link #CreateClient} to request a
 * client to Redis.
 */
public class RedisClusterClient extends BaseClient implements ClusterBaseCommands {

    protected RedisClusterClient(ConnectionManager connectionManager, CommandManager commandManager) {
        super(connectionManager, commandManager);
    }

    /**
     * Async request for an async (non-blocking) Redis client in Cluster mode.
     *
     * @param config Redis cluster client Configuration
     * @return a Future to connect and return a ClusterClient
     */
    public static CompletableFuture<RedisClusterClient> CreateClient(
            RedisClusterClientConfiguration config) {
        try {
            ChannelHandler channelHandler = buildChannelHandler();
            ConnectionManager connectionManager = buildConnectionManager(channelHandler);
            CommandManager commandManager = buildCommandManager(channelHandler);
            // TODO: Support exception throwing, including interrupted exceptions
            return connectionManager
                    .connectToRedis(config)
                    .thenApply(ignored -> new RedisClusterClient(connectionManager, commandManager));
        } catch (InterruptedException e) {
            // Something bad happened while we were establishing netty connection to UDS
            var future = new CompletableFuture<RedisClusterClient>();
            future.completeExceptionally(e);
            return future;
        }
    }

    @Override
    public CompletableFuture<ClusterValue<Object>> customCommand(String[] args) {
        Command command =
                Command.builder().requestType(Command.RequestType.CUSTOM_COMMAND).arguments(args).build();
        // TODO if a command returns a map as a single value, ClusterValue misleads user
        return commandManager.submitNewCommand(
                command, response -> ClusterValue.of(handleObjectResponse(response)));
    }

    @Override
    @SuppressWarnings("unchecked")
    public CompletableFuture<ClusterValue<Object>> customCommand(String[] args, Route route) {
        Command command =
                Command.builder()
                        .requestType(Command.RequestType.CUSTOM_COMMAND)
                        .arguments(args)
                        .route(route)
                        .build();

        return commandManager.submitNewCommand(
                command,
                response ->
                        route.isSingleNodeRoute()
                                ? ClusterValue.ofSingleValue(handleObjectResponse(response))
                                : ClusterValue.ofMultiValue((Map<String, Object>) handleObjectResponse(response)));
    }
}
