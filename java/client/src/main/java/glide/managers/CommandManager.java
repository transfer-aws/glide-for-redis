package glide.managers;

import glide.api.models.configuration.RequestRoutingConfiguration.Route;
import glide.api.models.configuration.RequestRoutingConfiguration.SimpleRoute;
import glide.api.models.configuration.RequestRoutingConfiguration.SlotIdRoute;
import glide.api.models.configuration.RequestRoutingConfiguration.SlotKeyRoute;
import glide.connectors.handlers.CallbackDispatcher;
import glide.connectors.handlers.ChannelHandler;
import glide.managers.models.Command;
import java.util.Optional;
import java.util.concurrent.CompletableFuture;
import lombok.RequiredArgsConstructor;
import redis_request.RedisRequestOuterClass;
import redis_request.RedisRequestOuterClass.Command.ArgsArray;
import redis_request.RedisRequestOuterClass.RedisRequest;
import redis_request.RedisRequestOuterClass.RequestType;
import response.ResponseOuterClass.Response;

/**
 * Service responsible for submitting command requests to a socket channel handler and unpack
 * responses from the same socket channel handler.
 */
@RequiredArgsConstructor
public class CommandManager {

    /** UDS connection representation. */
    private final ChannelHandler channel;

    /**
     * Build a command and send.
     *
     * @param command The command to execute
     * @param responseHandler The handler for the response object
     * @return A result promise of type T
     */
    public <T> CompletableFuture<T> submitNewCommand(
            Command command, RedisExceptionCheckedFunction<Response, T> responseHandler) {
        // write command request to channel
        // when complete, convert the response to our expected type T using the given responseHandler
        return channel
                .write(
                        prepareRedisRequest(
                                command.getRequestType(),
                                command.getArguments(),
                                Optional.ofNullable(command.getRoute())),
                        true)
                .thenApplyAsync(responseHandler::apply);
    }

    private RequestType mapRequestTypes(Command.RequestType inType) {
        switch (inType) {
            case CUSTOM_COMMAND:
                return RequestType.CustomCommand;
        }
        throw new RuntimeException("Unsupported request type");
    }

    /**
     * Build a protobuf command/transaction request object with routing options.<br>
     * Used by {@link CommandManager}.
     *
     * @param command Redis command type
     * @param args Redis command arguments
     * @param route Command routing parameters
     * @return An uncompleted request. {@link CallbackDispatcher} is responsible to complete it by
     *     adding a callback id.
     */
    private RedisRequest.Builder prepareRedisRequest(
            Command.RequestType command, String[] args, Optional<Route> route) {
        ArgsArray.Builder commandArgs = ArgsArray.newBuilder();
        for (var arg : args) {
            commandArgs.addArgs(arg);
        }

        var builder =
                RedisRequest.newBuilder()
                        .setSingleCommand(
                                RedisRequestOuterClass.Command.newBuilder()
                                        .setRequestType(mapRequestTypes(command))
                                        .setArgsArray(commandArgs.build())
                                        .build());

        if (route.isEmpty()) {
            return builder;
        }

        if (route.get() instanceof SimpleRoute) {
            builder.setRoute(
                    RedisRequestOuterClass.Routes.newBuilder()
                            .setSimpleRoutes(((SimpleRoute) route.get()).getProtobufMapping())
                            .build());
        } else if (route.get() instanceof SlotIdRoute) {
            builder.setRoute(
                    RedisRequestOuterClass.Routes.newBuilder()
                            .setSlotIdRoute(
                                    RedisRequestOuterClass.SlotIdRoute.newBuilder()
                                            .setSlotId(((SlotIdRoute) route.get()).getSlotId())
                                            .setSlotType(((SlotIdRoute) route.get()).getSlotType().getSlotTypes())));
        } else if (route.get() instanceof SlotKeyRoute) {
            builder.setRoute(
                    RedisRequestOuterClass.Routes.newBuilder()
                            .setSlotKeyRoute(
                                    RedisRequestOuterClass.SlotKeyRoute.newBuilder()
                                            .setSlotKey(((SlotKeyRoute) route.get()).getSlotKey())
                                            .setSlotType(((SlotKeyRoute) route.get()).getSlotType().getSlotTypes())));
        } else {
            throw new IllegalArgumentException("Unknown type of route");
        }
        return builder;
    }
}
