/** Copyright Valkey GLIDE Project Contributors - SPDX Identifier: Apache-2.0 */
package glide.standalone;

import static glide.TestConfiguration.SERVER_VERSION;
import static glide.TestUtilities.commonClientConfig;
import static glide.TestUtilities.getRandomString;
import static glide.api.BaseClient.OK;
import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertInstanceOf;
import static org.junit.jupiter.api.Assertions.assertNotNull;
import static org.junit.jupiter.api.Assertions.assertThrows;
import static org.junit.jupiter.api.Assertions.assertTrue;
import static org.junit.jupiter.api.Assumptions.assumeTrue;

import glide.api.GlideClient;
import glide.api.models.configuration.ServerCredentials;
import glide.api.models.exceptions.ClosingException;
import glide.api.models.exceptions.RequestException;
import java.util.Map;
import java.util.UUID;
import java.util.concurrent.ExecutionException;
import lombok.SneakyThrows;
import org.apache.commons.lang3.RandomStringUtils;
import org.junit.jupiter.api.Test;
import org.junit.jupiter.api.Timeout;
import org.junit.jupiter.params.ParameterizedTest;
import org.junit.jupiter.params.provider.ValueSource;

@Timeout(10) // seconds
public class StandaloneClientTests {

    @SneakyThrows
    @Test
    public void register_client_name_and_version() {
        String minVersion = "7.2.0";
        assumeTrue(
                SERVER_VERSION.isGreaterThanOrEqualTo(minVersion),
                "Valkey version required >= " + minVersion);

        GlideClient client = GlideClient.createClient(commonClientConfig().build()).get();

        String info = (String) client.customCommand(new String[] {"CLIENT", "INFO"}).get();
        assertTrue(info.contains("lib-name=GlideJava"));
        assertTrue(info.contains("lib-ver=unknown"));

        client.close();
    }

    @SneakyThrows
    @Test
    public void can_connect_with_auth_require_pass() {
        GlideClient client = GlideClient.createClient(commonClientConfig().build()).get();

        String password = "TEST_AUTH";
        client.configSet(Map.of("requirepass", password)).get();

        // Creation of a new client without a password should fail
        ExecutionException exception =
                assertThrows(
                        ExecutionException.class,
                        () -> GlideClient.createClient(commonClientConfig().build()).get());
        assertTrue(exception.getCause() instanceof ClosingException);

        // Creation of a new client with credentials
        GlideClient auth_client =
                GlideClient.createClient(
                                commonClientConfig()
                                        .credentials(ServerCredentials.builder().password(password).build())
                                        .build())
                        .get();

        String key = getRandomString(10);
        String value = getRandomString(10);

        assertEquals(OK, auth_client.set(key, value).get());
        assertEquals(value, auth_client.get(key).get());

        // Reset password
        client.configSet(Map.of("requirepass", "")).get();

        auth_client.close();
        client.close();
    }

    @SneakyThrows
    @Test
    public void can_connect_with_auth_acl() {
        GlideClient client = GlideClient.createClient(commonClientConfig().build()).get();

        String username = "testuser";
        String password = "TEST_AUTH";
        assertEquals(
                OK,
                client
                        .customCommand(
                                new String[] {
                                    "ACL",
                                    "SETUSER",
                                    username,
                                    "on",
                                    "allkeys",
                                    "+get",
                                    "+cluster",
                                    "+ping",
                                    "+info",
                                    "+client",
                                    ">" + password,
                                })
                        .get());

        String key = getRandomString(10);
        String value = getRandomString(10);

        assertEquals(OK, client.set(key, value).get());

        // Creation of a new client with credentials
        GlideClient testUserClient =
                GlideClient.createClient(
                                commonClientConfig()
                                        .credentials(
                                                ServerCredentials.builder().username(username).password(password).build())
                                        .build())
                        .get();

        assertEquals(value, testUserClient.get(key).get());
        ExecutionException executionException =
                assertThrows(ExecutionException.class, () -> testUserClient.set("foo", "bar").get());
        assertTrue(executionException.getCause() instanceof RequestException);

        client.customCommand(new String[] {"ACL", "DELUSER", username}).get();

        testUserClient.close();
        client.close();
    }

    @SneakyThrows
    @Test
    public void select_standalone_database_id() {
        GlideClient client = GlideClient.createClient(commonClientConfig().databaseId(4).build()).get();

        String clientInfo = (String) client.customCommand(new String[] {"CLIENT", "INFO"}).get();
        assertTrue(clientInfo.contains("db=4"));

        client.close();
    }

    @SneakyThrows
    @Test
    public void client_name() {
        GlideClient client =
                GlideClient.createClient(commonClientConfig().clientName("TEST_CLIENT_NAME").build()).get();

        String clientInfo = (String) client.customCommand(new String[] {"CLIENT", "INFO"}).get();
        assertTrue(clientInfo.contains("name=TEST_CLIENT_NAME"));

        client.close();
    }

    @Test
    @SneakyThrows
    public void closed_client_throws_ExecutionException_with_ClosingException_as_cause() {
        GlideClient client = GlideClient.createClient(commonClientConfig().build()).get();

        client.close();
        ExecutionException executionException =
                assertThrows(ExecutionException.class, () -> client.set("key", "value").get());
        assertTrue(executionException.getCause() instanceof ClosingException);
    }

    @SneakyThrows
    @Test
    public void test_update_connection_password_auth_non_valid_pass() {
        // Test Client fails on call to updateConnectionPassword with invalid parameters
        try (var testClient = GlideClient.createClient(commonClientConfig().build()).get()) {
            var emptyPasswordException =
                    assertThrows(
                            ExecutionException.class, () -> testClient.updateConnectionPassword("", true).get());
            assertInstanceOf(RequestException.class, emptyPasswordException.getCause());

            var noPasswordException =
                    assertThrows(
                            ExecutionException.class, () -> testClient.updateConnectionPassword(true).get());
            assertInstanceOf(RequestException.class, noPasswordException.getCause());
        }
    }

    @SneakyThrows
    @Test
    public void test_update_connection_password_no_server_auth() {
        var pwd = UUID.randomUUID().toString();

        try (var testClient = GlideClient.createClient(commonClientConfig().build()).get()) {
            // validate that we can use the client
            assertNotNull(testClient.info().get());

            // Test that immediate re-authentication fails when no server password is set.
            var exception =
                    assertThrows(
                            ExecutionException.class, () -> testClient.updateConnectionPassword(pwd, true).get());
            assertInstanceOf(RequestException.class, exception.getCause());
        }
    }

    @SneakyThrows
    @Test
    public void test_update_connection_password_long() {
        var pwd = RandomStringUtils.randomAlphabetic(1000);

        try (var testClient = GlideClient.createClient(commonClientConfig().build()).get()) {
            // validate that we can use the client
            assertNotNull(testClient.info().get());

            // Test replacing connection password with a long password string.
            assertEquals(OK, testClient.updateConnectionPassword(pwd, false).get());
        }
    }

    @Timeout(50)
    @SneakyThrows
    @Test
    public void test_replace_password_immediateAuth_wrong_password() {
        var pwd = UUID.randomUUID().toString();
        var notThePwd = UUID.randomUUID().toString();

        GlideClient adminClient = GlideClient.createClient(commonClientConfig().build()).get();
        try (var testClient = GlideClient.createClient(commonClientConfig().build()).get()) {
            // validate that we can use the client
            assertNotNull(testClient.info().get());

            // set the password to something else
            adminClient.configSet(Map.of("requirepass", notThePwd)).get();

            // Test that re-authentication fails when using wrong password.
            var exception =
                    assertThrows(
                            ExecutionException.class, () -> testClient.updateConnectionPassword(pwd, true).get());
            assertInstanceOf(RequestException.class, exception.getCause());

            // But using something else password returns OK
            assertEquals(OK, testClient.updateConnectionPassword(notThePwd, true).get());
        } finally {
            adminClient.configSet(Map.of("requirepass", "")).get();
            adminClient.close();
        }
    }

    @SneakyThrows
    @ParameterizedTest
    @ValueSource(booleans = {true, false})
    public void test_update_connection_password_connection_lost_before_password_update(
            boolean immediateAuth) {
        GlideClient adminClient = GlideClient.createClient(commonClientConfig().build()).get();
        var pwd = UUID.randomUUID().toString();

        try (var testClient = GlideClient.createClient(commonClientConfig().build()).get()) {
            // validate that we can use the client
            assertNotNull(testClient.info().get());

            // set the password and forcefully drop connection for the testClient
            assertEquals("OK", adminClient.configSet(Map.of("requirepass", pwd)).get());
            adminClient.customCommand(new String[] {"CLIENT", "KILL", "TYPE", "NORMAL"}).get();

            /*
             * Some explanation for the curious mind:
             * Our library is abstracting a connection or connections, with a lot of mechanism around it, making it behave like what we call a "client".
             * When using standalone mode, the client is a single connection, so on disconnection the first thing it planned to do is to reconnect.
             *
             * There's no reason to get other commands and to take care of them since to serve commands we need to be connected.
             *
             * Hence, the client will try to reconnect and will not listen try to take care of new tasks, but will let them wait in line,
             * so the update connection password will not be able to reach the connection and will return an error.
             * For future versions, standalone will be considered as a different animal then it is now, since standalone is not necessarily one node.
             * It can be replicated and have a lot of nodes, and to be what we like to call "one shard cluster".
             * So, in the future, we will have many existing connection and request can be managed also when one connection is locked,
             */
            var exception =
                    assertThrows(
                            ExecutionException.class,
                            () -> testClient.updateConnectionPassword(pwd, immediateAuth).get());
        } finally {
            adminClient.configSet(Map.of("requirepass", "")).get();
            adminClient.close();
        }
    }
}
