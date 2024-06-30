import json
import random
import string
from typing import Any, Dict, List, Mapping, Optional, TypeVar, Union, cast

from glide.async_commands.core import InfoSection
from glide.async_commands.utils.utils import convert_bytes_to_string_dict
from glide.constants import TResult
from glide.glide_client import TGlideClient
from packaging import version

T = TypeVar("T")


def is_single_response(response: T, single_res: T) -> bool:
    """
    Recursively checks if a given response matches the type structure of single_res.

    Args:
        response (T): The response to check.
        single_res (T): An object with the expected type structure as an example for the single node response.

    Returns:
        bool: True if response matches the structure of single_res, False otherwise.

     Example:
        >>> is_single_response(["value"], LIST_STR)
        True
        >>> is_single_response([["value"]], LIST_STR)
        False
    """
    if isinstance(single_res, list) and isinstance(response, list):
        return is_single_response(response[0], single_res[0])
    elif isinstance(response, type(single_res)):
        return True
    return False


def get_first_result(
    res: TResult,
) -> bytes:
    while isinstance(res, list):
        res = (
            res[1]
            if not isinstance(res[0], list) and res[0].startswith("127.0.0.1")
            else res[0]
        )

    if isinstance(res, dict):
        res = list(res.values())[0]
    return cast(bytes, res)


def parse_info_response(res: Union[bytes, Dict[bytes, bytes]]) -> Dict[str, str]:
    res_first = get_first_result(res)
    res_decoded = res_first.decode() if isinstance(res_first, bytes) else res_first
    info_lines = [
        line for line in res_decoded.splitlines() if line and not line.startswith("#")
    ]
    info_dict = {}
    for line in info_lines:
        splitted_line = line.split(":")
        key = splitted_line[0]
        value = splitted_line[1]
        info_dict[key] = value
    return info_dict


def get_random_string(length):
    result_str = "".join(random.choice(string.ascii_letters) for i in range(length))
    return result_str


async def check_if_server_version_lt(client: TGlideClient, min_version: str) -> bool:
    # TODO: change it to pytest fixture after we'll implement a sync client
    info_str = await client.info([InfoSection.SERVER])
    redis_version = parse_info_response(info_str).get("redis_version")
    assert redis_version is not None
    return version.parse(redis_version) < version.parse(min_version)


def compare_maps(
    map1: Optional[
        Union[
            Mapping[str, TResult],
            Dict[str, TResult],
            Mapping[bytes, TResult],
            Dict[bytes, TResult],
        ]
    ],
    map2: Optional[
        Union[
            Mapping[str, TResult],
            Dict[str, TResult],
            Mapping[bytes, TResult],
            Dict[bytes, TResult],
        ]
    ],
) -> bool:
    """
    Compare two maps by converting them to JSON strings and checking for equality, including property order.

    Args:
        map1 (Optional[Union[Mapping[Union[str, bytes], TResult], Dict[Union[str, bytes], TResult]]]): The first map to compare.
        map2 (Optional[Union[Mapping[Union[str, bytes], TResult], Dict[Union[str, bytes], TResult]]]): The second map to compare.

    Returns:
        bool: True if the maps are equal, False otherwise.

    Notes:
        This function compares two maps, including their property order.
        It checks that each key-value pair in `map1` is equal to the corresponding key-value pair in `map2`,
        and ensures that the order of properties is also the same.
        Direct comparison with `assert map1 == map2` might ignore the order of properties.

    Example:
        mapA = {'name': 'John', 'age': 30}
        mapB = {'age': 30, 'name': 'John'}

        # Direct comparison will pass because it ignores property order
        assert mapA == mapB  # This will pass

        # Correct comparison using compare_maps function
        compare_maps(mapA, mapB)  # This will return False due to different property order
    """
    if map1 is None and map2 is None:
        return True
    if map1 is None or map2 is None:
        return False
    return json.dumps(convert_bytes_to_string_dict(map1)) == json.dumps(
        convert_bytes_to_string_dict(map2)
    )


def convert_str_to_bytes_list(lst):
    return [
        (
            elem.encode("utf-8")
            if isinstance(elem, str)
            else convert_str_to_bytes_list(elem) if isinstance(elem, list) else elem
        )
        for elem in lst
    ]


def convert_str_to_bytes_set(set):
    return {elem.encode("utf-8") if isinstance(elem, str) else elem for elem in set}


def generate_lua_lib_code(
    lib_name: str, functions: Mapping[str, str], readonly: bool
) -> str:
    code = f"#!lua name={lib_name}\n"
    for function_name, function_body in functions.items():
        code += (
            f"redis.register_function{{ function_name = '{function_name}', callback = function(keys, args) "
            f"{function_body} end"
        )
        if readonly:
            code += ", flags = { 'no-writes' }"
        code += " }\n"
    return code
