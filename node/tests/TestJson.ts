/**
 * Copyright Valkey GLIDE Project Contributors - SPDX Identifier: Apache-2.0
 */


export function runJsonTests(config: {
    init: () => Promise<{ client: Client }>;
    close: (testSucceeded: boolean) => void;
    timeout?: number;
}) {
    const runTest = async (test: (client: Client) => Promise<void>) => {
        const { client } = await config.init();
        let testSucceeded = false;

        try {
            await test(client);
            testSucceeded = true;
        } finally {
            config.close(testSucceeded);
        }
    };

    it(
        "test1",
        async () => {
            await runTest((client: Client) => (client));
        },
        config.timeout,
    );
}


// @pytest.mark.asyncio
// class TestJson:
//     @pytest.mark.parametrize("cluster_mode", [True, False])
//     @pytest.mark.parametrize("protocol", [ProtocolVersion.RESP2, ProtocolVersion.RESP3])
//     async def test_json_module_is_loaded(self, redis_client: TRedisClient):
//         res = parse_info_response(await redis_client.info([InfoSection.MODULES]))
//         assert "ReJSON" in res["module"]

//     @pytest.mark.parametrize("cluster_mode", [True, False])
//     @pytest.mark.parametrize("protocol", [ProtocolVersion.RESP2, ProtocolVersion.RESP3])
//     async def test_json_set_get(self, redis_client: TRedisClient):
//         key = get_random_string(5)

//         json_value = {"a": 1.0, "b": 2}
//         assert await json.set(redis_client, key, "$", OuterJson.dumps(json_value)) == OK

//         result = await json.get(redis_client, key, ".")
//         assert isinstance(result, str)
//         assert OuterJson.loads(result) == json_value

//         result = await json.get(redis_client, key, ["$.a", "$.b"])
//         assert isinstance(result, str)
//         assert OuterJson.loads(result) == {"$.a": [1.0], "$.b": [2]}

//         assert await json.get(redis_client, "non_existing_key", "$") is None
//         assert await json.get(redis_client, key, "$.d") == "[]"

//     @pytest.mark.parametrize("cluster_mode", [True, False])
//     @pytest.mark.parametrize("protocol", [ProtocolVersion.RESP2, ProtocolVersion.RESP3])
//     async def test_json_set_get_multiple_values(self, redis_client: TRedisClient):
//         key = get_random_string(5)

//         assert (
//             await json.set(
//                 redis_client,
//                 key,
//                 "$",
//                 OuterJson.dumps({"a": {"c": 1, "d": 4}, "b": {"c": 2}, "c": True}),
//             )
//             == OK
//         )

//         result = await json.get(redis_client, key, "$..c")
//         assert isinstance(result, str)
//         assert OuterJson.loads(result) == [True, 1, 2]

//         result = await json.get(redis_client, key, ["$..c", "$.c"])
//         assert isinstance(result, str)
//         assert OuterJson.loads(result) == {"$..c": [True, 1, 2], "$.c": [True]}

//         assert await json.set(redis_client, key, "$..c", '"new_value"') == OK
//         result = await json.get(redis_client, key, "$..c")
//         assert isinstance(result, str)
//         assert OuterJson.loads(result) == ["new_value"] * 3

//     @pytest.mark.parametrize("cluster_mode", [True, False])
//     @pytest.mark.parametrize("protocol", [ProtocolVersion.RESP2, ProtocolVersion.RESP3])
//     async def test_json_set_conditional_set(self, redis_client: TRedisClient):
//         key = get_random_string(5)
//         value = OuterJson.dumps({"a": 1.0, "b": 2})
//         assert (
//             await json.set(
//                 redis_client,
//                 key,
//                 "$",
//                 value,
//                 ConditionalChange.ONLY_IF_EXISTS,
//             )
//             is None
//         )
//         assert (
//             await json.set(
//                 redis_client,
//                 key,
//                 "$",
//                 value,
//                 ConditionalChange.ONLY_IF_DOES_NOT_EXIST,
//             )
//             == OK
//         )

//         assert (
//             await json.set(
//                 redis_client,
//                 key,
//                 "$.a",
//                 "4.5",
//                 ConditionalChange.ONLY_IF_DOES_NOT_EXIST,
//             )
//             is None
//         )

//         assert await json.get(redis_client, key, ".a") == "1.0"

//         assert (
//             await json.set(
//                 redis_client,
//                 key,
//                 "$.a",
//                 "4.5",
//                 ConditionalChange.ONLY_IF_EXISTS,
//             )
//             == OK
//         )

//         assert await json.get(redis_client, key, ".a") == "4.5"

//     @pytest.mark.parametrize("cluster_mode", [True, False])
//     @pytest.mark.parametrize("protocol", [ProtocolVersion.RESP2, ProtocolVersion.RESP3])
//     async def test_json_get_formatting(self, redis_client: TRedisClient):
//         key = get_random_string(5)
//         assert (
//             await json.set(
//                 redis_client,
//                 key,
//                 "$",
//                 OuterJson.dumps({"a": 1.0, "b": 2, "c": {"d": 3, "e": 4}}),
//             )
//             == OK
//         )

//         result = await json.get(
//             redis_client, key, "$", JsonGetOptions(indent="  ", newline="\n", space=" ")
//         )

//         expected_result = '[\n  {\n    "a": 1.0,\n    "b": 2,\n    "c": {\n      "d": 3,\n      "e": 4\n    }\n  }\n]'
//         assert result == expected_result

//         result = await json.get(
//             redis_client, key, "$", JsonGetOptions(indent="~", newline="\n", space="*")
//         )

//         expected_result = (
//             '[\n~{\n~~"a":*1.0,\n~~"b":*2,\n~~"c":*{\n~~~"d":*3,\n~~~"e":*4\n~~}\n~}\n]'
//         )
//         assert result == expected_result

// TODO: convert to ts tests
