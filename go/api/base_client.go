// Copyright Valkey GLIDE Project Contributors - SPDX Identifier: Apache-2.0

package api

// #cgo LDFLAGS: -L../target/release -lglide_rs
// #include "../lib.h"
//
// void successCallback(void *channelPtr, struct CommandResponse *message);
// void failureCallback(void *channelPtr, char *errMessage, RequestErrorType errType);
import "C"

import (
	"errors"
	"math"
	"slices"
	"strconv"
	"unsafe"

	"github.com/valkey-io/valkey-glide/go/glide/protobuf"
	"github.com/valkey-io/valkey-glide/go/glide/utils"
	"google.golang.org/protobuf/proto"
)

// BaseClient defines an interface for methods common to both [GlideClient] and [GlideClusterClient].
type BaseClient interface {
	StringCommands
	HashCommands
	ListCommands
	SetCommands
	ConnectionManagementCommands
	GenericBaseCommands
	// Close terminates the client by closing all associated resources.
	Close()
}

const OK = "OK"

type payload struct {
	value *C.struct_CommandResponse
	error error
}

//export successCallback
func successCallback(channelPtr unsafe.Pointer, cResponse *C.struct_CommandResponse) {
	response := cResponse
	resultChannel := *(*chan payload)(channelPtr)
	resultChannel <- payload{value: response, error: nil}
}

//export failureCallback
func failureCallback(channelPtr unsafe.Pointer, cErrorMessage *C.char, cErrorType C.RequestErrorType) {
	resultChannel := *(*chan payload)(channelPtr)
	resultChannel <- payload{value: nil, error: goError(cErrorType, cErrorMessage)}
}

type clientConfiguration interface {
	toProtobuf() *protobuf.ConnectionRequest
}

type baseClient struct {
	coreClient unsafe.Pointer
}

// Creates a connection by invoking the `create_client` function from Rust library via FFI.
// Passes the pointers to callback functions which will be invoked when the command succeeds or fails.
// Once the connection is established, this function invokes `free_connection_response` exposed by rust library to free the
// connection_response to avoid any memory leaks.
func createClient(config clientConfiguration) (*baseClient, error) {
	request := config.toProtobuf()
	msg, err := proto.Marshal(request)
	if err != nil {
		return nil, err
	}

	byteCount := len(msg)
	requestBytes := C.CBytes(msg)
	cResponse := (*C.struct_ConnectionResponse)(
		C.create_client(
			(*C.uchar)(requestBytes),
			C.uintptr_t(byteCount),
			(C.SuccessCallback)(unsafe.Pointer(C.successCallback)),
			(C.FailureCallback)(unsafe.Pointer(C.failureCallback)),
		),
	)
	defer C.free_connection_response(cResponse)

	cErr := cResponse.connection_error_message
	if cErr != nil {
		message := C.GoString(cErr)
		return nil, &ConnectionError{message}
	}

	return &baseClient{cResponse.conn_ptr}, nil
}

// Close terminates the client by closing all associated resources.
func (client *baseClient) Close() {
	if client.coreClient == nil {
		return
	}

	C.close_client(client.coreClient)
	client.coreClient = nil
}

func (client *baseClient) executeCommand(requestType C.RequestType, args []string) (*C.struct_CommandResponse, error) {
	if client.coreClient == nil {
		return nil, &ClosingError{"ExecuteCommand failed. The client is closed."}
	}

	var cArgsPtr *C.uintptr_t = nil
	var argLengthsPtr *C.ulong = nil
	if len(args) > 0 {
		cArgs, argLengths := toCStrings(args)
		cArgsPtr = &cArgs[0]
		argLengthsPtr = &argLengths[0]
	}

	resultChannel := make(chan payload)
	resultChannelPtr := uintptr(unsafe.Pointer(&resultChannel))

	C.command(
		client.coreClient,
		C.uintptr_t(resultChannelPtr),
		uint32(requestType),
		C.size_t(len(args)),
		cArgsPtr,
		argLengthsPtr,
	)
	payload := <-resultChannel
	if payload.error != nil {
		return nil, payload.error
	}
	return payload.value, nil
}

// Zero copying conversion from go's []string into C pointers
func toCStrings(args []string) ([]C.uintptr_t, []C.ulong) {
	cStrings := make([]C.uintptr_t, len(args))
	stringLengths := make([]C.ulong, len(args))
	for i, str := range args {
		bytes := utils.StringToBytes(str)
		var ptr uintptr
		if len(str) > 0 {
			ptr = uintptr(unsafe.Pointer(&bytes[0]))
		}
		cStrings[i] = C.uintptr_t(ptr)
		stringLengths[i] = C.size_t(len(str))
	}
	return cStrings, stringLengths
}

func (client *baseClient) Set(key string, value string) (Result[string], error) {
	result, err := client.executeCommand(C.Set, []string{key, value})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) SetWithOptions(key string, value string, options *SetOptions) (Result[string], error) {
	optionArgs, err := options.toArgs()
	if err != nil {
		return CreateNilStringResult(), err
	}

	result, err := client.executeCommand(C.Set, append([]string{key, value}, optionArgs...))
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) Get(key string) (Result[string], error) {
	result, err := client.executeCommand(C.Get, []string{key})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) GetEx(key string) (Result[string], error) {
	result, err := client.executeCommand(C.GetEx, []string{key})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) GetExWithOptions(key string, options *GetExOptions) (Result[string], error) {
	optionArgs, err := options.toArgs()
	if err != nil {
		return CreateNilStringResult(), err
	}

	result, err := client.executeCommand(C.GetEx, append([]string{key}, optionArgs...))
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) MSet(keyValueMap map[string]string) (Result[string], error) {
	result, err := client.executeCommand(C.MSet, utils.MapToString(keyValueMap))
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) MSetNX(keyValueMap map[string]string) (Result[bool], error) {
	result, err := client.executeCommand(C.MSetNX, utils.MapToString(keyValueMap))
	if err != nil {
		return CreateNilBoolResult(), err
	}

	return handleBooleanResponse(result)
}

func (client *baseClient) MGet(keys []string) ([]Result[string], error) {
	result, err := client.executeCommand(C.MGet, keys)
	if err != nil {
		return nil, err
	}

	return handleStringArrayResponse(result)
}

func (client *baseClient) Incr(key string) (Result[int64], error) {
	result, err := client.executeCommand(C.Incr, []string{key})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) IncrBy(key string, amount int64) (Result[int64], error) {
	result, err := client.executeCommand(C.IncrBy, []string{key, utils.IntToString(amount)})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) IncrByFloat(key string, amount float64) (Result[float64], error) {
	result, err := client.executeCommand(
		C.IncrByFloat,
		[]string{key, utils.FloatToString(amount)},
	)
	if err != nil {
		return CreateNilFloat64Result(), err
	}

	return handleDoubleResponse(result)
}

func (client *baseClient) Decr(key string) (Result[int64], error) {
	result, err := client.executeCommand(C.Decr, []string{key})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) DecrBy(key string, amount int64) (Result[int64], error) {
	result, err := client.executeCommand(C.DecrBy, []string{key, utils.IntToString(amount)})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) Strlen(key string) (Result[int64], error) {
	result, err := client.executeCommand(C.Strlen, []string{key})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SetRange(key string, offset int, value string) (Result[int64], error) {
	result, err := client.executeCommand(C.SetRange, []string{key, strconv.Itoa(offset), value})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) GetRange(key string, start int, end int) (Result[string], error) {
	result, err := client.executeCommand(C.GetRange, []string{key, strconv.Itoa(start), strconv.Itoa(end)})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) Append(key string, value string) (Result[int64], error) {
	result, err := client.executeCommand(C.Append, []string{key, value})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) LCS(key1 string, key2 string) (Result[string], error) {
	result, err := client.executeCommand(C.LCS, []string{key1, key2})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) GetDel(key string) (Result[string], error) {
	if key == "" {
		return CreateNilStringResult(), errors.New("key is required")
	}

	result, err := client.executeCommand(C.GetDel, []string{key})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) HGet(key string, field string) (Result[string], error) {
	result, err := client.executeCommand(C.HGet, []string{key, field})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) HGetAll(key string) (map[Result[string]]Result[string], error) {
	result, err := client.executeCommand(C.HGetAll, []string{key})
	if err != nil {
		return nil, err
	}

	return handleStringToStringMapResponse(result)
}

func (client *baseClient) HMGet(key string, fields []string) ([]Result[string], error) {
	result, err := client.executeCommand(C.HMGet, append([]string{key}, fields...))
	if err != nil {
		return nil, err
	}

	return handleStringArrayResponse(result)
}

func (client *baseClient) HSet(key string, values map[string]string) (Result[int64], error) {
	result, err := client.executeCommand(C.HSet, utils.ConvertMapToKeyValueStringArray(key, values))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) HSetNX(key string, field string, value string) (Result[bool], error) {
	result, err := client.executeCommand(C.HSetNX, []string{key, field, value})
	if err != nil {
		return CreateNilBoolResult(), err
	}

	return handleBooleanResponse(result)
}

func (client *baseClient) HDel(key string, fields []string) (Result[int64], error) {
	result, err := client.executeCommand(C.HDel, append([]string{key}, fields...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) HLen(key string) (Result[int64], error) {
	result, err := client.executeCommand(C.HLen, []string{key})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) HVals(key string) ([]Result[string], error) {
	result, err := client.executeCommand(C.HVals, []string{key})
	if err != nil {
		return nil, err
	}

	return handleStringArrayResponse(result)
}

func (client *baseClient) HExists(key string, field string) (Result[bool], error) {
	result, err := client.executeCommand(C.HExists, []string{key, field})
	if err != nil {
		return CreateNilBoolResult(), err
	}

	return handleBooleanResponse(result)
}

func (client *baseClient) HKeys(key string) ([]Result[string], error) {
	result, err := client.executeCommand(C.HKeys, []string{key})
	if err != nil {
		return nil, err
	}

	return handleStringArrayResponse(result)
}

func (client *baseClient) HStrLen(key string, field string) (Result[int64], error) {
	result, err := client.executeCommand(C.HStrlen, []string{key, field})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) LPush(key string, elements []string) (Result[int64], error) {
	result, err := client.executeCommand(C.LPush, append([]string{key}, elements...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) LPop(key string) (Result[string], error) {
	result, err := client.executeCommand(C.LPop, []string{key})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) LPopCount(key string, count int64) ([]Result[string], error) {
	result, err := client.executeCommand(C.LPop, []string{key, utils.IntToString(count)})
	if err != nil {
		return nil, err
	}

	return handleStringArrayOrNullResponse(result)
}

func (client *baseClient) LPos(key string, element string) (Result[int64], error) {
	result, err := client.executeCommand(C.LPos, []string{key, element})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongOrNullResponse(result)
}

func (client *baseClient) LPosWithOptions(key string, element string, options *LPosOptions) (Result[int64], error) {
	result, err := client.executeCommand(C.LPos, append([]string{key, element}, options.toArgs()...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongOrNullResponse(result)
}

func (client *baseClient) LPosCount(key string, element string, count int64) ([]Result[int64], error) {
	result, err := client.executeCommand(C.LPos, []string{key, element, CountKeyword, utils.IntToString(count)})
	if err != nil {
		return nil, err
	}

	return handleLongArrayResponse(result)
}

func (client *baseClient) LPosCountWithOptions(
	key string,
	element string,
	count int64,
	options *LPosOptions,
) ([]Result[int64], error) {
	result, err := client.executeCommand(
		C.LPos,
		append([]string{key, element, CountKeyword, utils.IntToString(count)}, options.toArgs()...),
	)
	if err != nil {
		return nil, err
	}

	return handleLongArrayResponse(result)
}

func (client *baseClient) RPush(key string, elements []string) (Result[int64], error) {
	result, err := client.executeCommand(C.RPush, append([]string{key}, elements...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SAdd(key string, members []string) (Result[int64], error) {
	result, err := client.executeCommand(C.SAdd, append([]string{key}, members...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SRem(key string, members []string) (Result[int64], error) {
	result, err := client.executeCommand(C.SRem, append([]string{key}, members...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SMembers(key string) (map[Result[string]]struct{}, error) {
	result, err := client.executeCommand(C.SMembers, []string{key})
	if err != nil {
		return nil, err
	}

	return handleStringSetResponse(result)
}

func (client *baseClient) SCard(key string) (Result[int64], error) {
	result, err := client.executeCommand(C.SCard, []string{key})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SIsMember(key string, member string) (Result[bool], error) {
	result, err := client.executeCommand(C.SIsMember, []string{key, member})
	if err != nil {
		return CreateNilBoolResult(), err
	}

	return handleBooleanResponse(result)
}

func (client *baseClient) SDiff(keys []string) (map[Result[string]]struct{}, error) {
	result, err := client.executeCommand(C.SDiff, keys)
	if err != nil {
		return nil, err
	}

	return handleStringSetResponse(result)
}

func (client *baseClient) SDiffStore(destination string, keys []string) (Result[int64], error) {
	result, err := client.executeCommand(C.SDiffStore, append([]string{destination}, keys...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SInter(keys []string) (map[Result[string]]struct{}, error) {
	result, err := client.executeCommand(C.SInter, keys)
	if err != nil {
		return nil, err
	}

	return handleStringSetResponse(result)
}

func (client *baseClient) SInterCard(keys []string) (Result[int64], error) {
	result, err := client.executeCommand(C.SInterCard, append([]string{strconv.Itoa(len(keys))}, keys...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SInterCardLimit(keys []string, limit int64) (Result[int64], error) {
	args := slices.Concat([]string{utils.IntToString(int64(len(keys)))}, keys, []string{"LIMIT", utils.IntToString(limit)})

	result, err := client.executeCommand(C.SInterCard, args)
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) SRandMember(key string) (Result[string], error) {
	result, err := client.executeCommand(C.SRandMember, []string{key})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) SPop(key string) (Result[string], error) {
	result, err := client.executeCommand(C.SPop, []string{key})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) LRange(key string, start int64, end int64) ([]Result[string], error) {
	result, err := client.executeCommand(C.LRange, []string{key, utils.IntToString(start), utils.IntToString(end)})
	if err != nil {
		return nil, err
	}

	return handleStringArrayResponse(result)
}

func (client *baseClient) LIndex(key string, index int64) (Result[string], error) {
	result, err := client.executeCommand(C.LIndex, []string{key, utils.IntToString(index)})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) LTrim(key string, start int64, end int64) (Result[string], error) {
	result, err := client.executeCommand(C.LTrim, []string{key, utils.IntToString(start), utils.IntToString(end)})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) LLen(key string) (Result[int64], error) {
	result, err := client.executeCommand(C.LLen, []string{key})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) LRem(key string, count int64, element string) (Result[int64], error) {
	result, err := client.executeCommand(C.LRem, []string{key, utils.IntToString(count), element})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) RPop(key string) (Result[string], error) {
	result, err := client.executeCommand(C.RPop, []string{key})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) RPopCount(key string, count int64) ([]Result[string], error) {
	result, err := client.executeCommand(C.RPop, []string{key, utils.IntToString(count)})
	if err != nil {
		return nil, err
	}

	return handleStringArrayOrNullResponse(result)
}

func (client *baseClient) LInsert(
	key string,
	insertPosition InsertPosition,
	pivot string,
	element string,
) (Result[int64], error) {
	insertPositionStr, err := insertPosition.toString()
	if err != nil {
		return CreateNilInt64Result(), err
	}

	result, err := client.executeCommand(C.LInsert, []string{key, insertPositionStr, pivot, element})
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) BLPop(keys []string, timeoutSecs float64) ([]Result[string], error) {
	result, err := client.executeCommand(C.BLPop, append(keys, utils.FloatToString(timeoutSecs)))
	if err != nil {
		return nil, err
	}

	return handleStringArrayOrNullResponse(result)
}

func (client *baseClient) BRPop(keys []string, timeoutSecs float64) ([]Result[string], error) {
	result, err := client.executeCommand(C.BRPop, append(keys, utils.FloatToString(timeoutSecs)))
	if err != nil {
		return nil, err
	}

	return handleStringArrayOrNullResponse(result)
}

func (client *baseClient) RPushX(key string, elements []string) (Result[int64], error) {
	result, err := client.executeCommand(C.RPushX, append([]string{key}, elements...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) LPushX(key string, elements []string) (Result[int64], error) {
	result, err := client.executeCommand(C.LPushX, append([]string{key}, elements...))
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}

func (client *baseClient) LMPop(keys []string, listDirection ListDirection) (map[Result[string]][]Result[string], error) {
	listDirectionStr, err := listDirection.toString()
	if err != nil {
		return nil, err
	}

	// Check for potential length overflow.
	if len(keys) > math.MaxInt-2 {
		return nil, &RequestError{"Length overflow for the provided keys"}
	}

	// args slice will have 2 more arguments with the keys provided.
	args := make([]string, 0, len(keys)+2)
	args = append(args, strconv.Itoa(len(keys)))
	args = append(args, keys...)
	args = append(args, listDirectionStr)
	result, err := client.executeCommand(C.LMPop, args)
	if err != nil {
		return nil, err
	}

	return handleStringToStringArrayMapOrNullResponse(result)
}

func (client *baseClient) LMPopCount(
	keys []string,
	listDirection ListDirection,
	count int64,
) (map[Result[string]][]Result[string], error) {
	listDirectionStr, err := listDirection.toString()
	if err != nil {
		return nil, err
	}

	// Check for potential length overflow.
	if len(keys) > math.MaxInt-4 {
		return nil, &RequestError{"Length overflow for the provided keys"}
	}

	// args slice will have 4 more arguments with the keys provided.
	args := make([]string, 0, len(keys)+4)
	args = append(args, strconv.Itoa(len(keys)))
	args = append(args, keys...)
	args = append(args, listDirectionStr, CountKeyword, utils.IntToString(count))
	result, err := client.executeCommand(C.LMPop, args)
	if err != nil {
		return nil, err
	}

	return handleStringToStringArrayMapOrNullResponse(result)
}

func (client *baseClient) BLMPop(
	keys []string,
	listDirection ListDirection,
	timeoutSecs float64,
) (map[Result[string]][]Result[string], error) {
	listDirectionStr, err := listDirection.toString()
	if err != nil {
		return nil, err
	}

	// Check for potential length overflow.
	if len(keys) > math.MaxInt-3 {
		return nil, &RequestError{"Length overflow for the provided keys"}
	}

	// args slice will have 3 more arguments with the keys provided.
	args := make([]string, 0, len(keys)+3)
	args = append(args, utils.FloatToString(timeoutSecs), strconv.Itoa(len(keys)))
	args = append(args, keys...)
	args = append(args, listDirectionStr)
	result, err := client.executeCommand(C.BLMPop, args)
	if err != nil {
		return nil, err
	}

	return handleStringToStringArrayMapOrNullResponse(result)
}

func (client *baseClient) BLMPopCount(
	keys []string,
	listDirection ListDirection,
	count int64,
	timeoutSecs float64,
) (map[Result[string]][]Result[string], error) {
	listDirectionStr, err := listDirection.toString()
	if err != nil {
		return nil, err
	}

	// Check for potential length overflow.
	if len(keys) > math.MaxInt-5 {
		return nil, &RequestError{"Length overflow for the provided keys"}
	}

	// args slice will have 5 more arguments with the keys provided.
	args := make([]string, 0, len(keys)+5)
	args = append(args, utils.FloatToString(timeoutSecs), strconv.Itoa(len(keys)))
	args = append(args, keys...)
	args = append(args, listDirectionStr, CountKeyword, utils.IntToString(count))
	result, err := client.executeCommand(C.BLMPop, args)
	if err != nil {
		return nil, err
	}

	return handleStringToStringArrayMapOrNullResponse(result)
}

func (client *baseClient) LSet(key string, index int64, element string) (Result[string], error) {
	result, err := client.executeCommand(C.LSet, []string{key, utils.IntToString(index), element})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringResponse(result)
}

func (client *baseClient) LMove(
	source string,
	destination string,
	whereFrom ListDirection,
	whereTo ListDirection,
) (Result[string], error) {
	whereFromStr, err := whereFrom.toString()
	if err != nil {
		return CreateNilStringResult(), err
	}
	whereToStr, err := whereTo.toString()
	if err != nil {
		return CreateNilStringResult(), err
	}

	result, err := client.executeCommand(C.LMove, []string{source, destination, whereFromStr, whereToStr})
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) BLMove(
	source string,
	destination string,
	whereFrom ListDirection,
	whereTo ListDirection,
	timeoutSecs float64,
) (Result[string], error) {
	whereFromStr, err := whereFrom.toString()
	if err != nil {
		return CreateNilStringResult(), err
	}
	whereToStr, err := whereTo.toString()
	if err != nil {
		return CreateNilStringResult(), err
	}

	result, err := client.executeCommand(
		C.BLMove,
		[]string{source, destination, whereFromStr, whereToStr, utils.FloatToString(timeoutSecs)},
	)
	if err != nil {
		return CreateNilStringResult(), err
	}

	return handleStringOrNullResponse(result)
}

func (client *baseClient) Ping() (string, error) {
	result, err := client.executeCommand(C.Ping, []string{})
	if err != nil {
		return "", err
	}

	response, err := handleStringResponse(result)
	if err != nil {
		return "", err
	}
	return response.Value(), nil
}

func (client *baseClient) PingWithMessage(message string) (string, error) {
	args := []string{message}

	result, err := client.executeCommand(C.Ping, args)
	if err != nil {
		return "", err
	}

	response, err := handleStringResponse(result)
	if err != nil {
		return "", err
	}
	return response.Value(), nil
}

func (client *baseClient) Del(keys []string) (Result[int64], error) {
	result, err := client.executeCommand(C.Del, keys)
	if err != nil {
		return CreateNilInt64Result(), err
	}

	return handleLongResponse(result)
}
