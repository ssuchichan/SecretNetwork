import protobuf from "protobufjs/minimal";

let descriptor =
  '{"nested":{"cosmos":{"nested":{"base":{"nested":{"abci":{"nested":{"v1beta1":{"options":{"go_package":"github.com/cosmos/cosmos-sdk/types","(gogoproto.goproto_stringer_all)":false},"nested":{"TxResponse":{"options":{"(gogoproto.goproto_getters)":false},"fields":{"height":{"type":"int64","id":1},"txhash":{"type":"string","id":2,"options":{"(gogoproto.customname)":"TxHash"}},"codespace":{"type":"string","id":3},"code":{"type":"uint32","id":4},"data":{"type":"string","id":5},"rawLog":{"type":"string","id":6},"logs":{"rule":"repeated","type":"ABCIMessageLog","id":7,"options":{"(gogoproto.castrepeated)":"ABCIMessageLogs","(gogoproto.nullable)":false}},"info":{"type":"string","id":8},"gasWanted":{"type":"int64","id":9},"gasUsed":{"type":"int64","id":10},"tx":{"type":"google.protobuf.Any","id":11},"timestamp":{"type":"string","id":12}}},"ABCIMessageLog":{"options":{"(gogoproto.stringer)":true},"fields":{"msgIndex":{"type":"uint32","id":1},"log":{"type":"string","id":2},"events":{"rule":"repeated","type":"StringEvent","id":3,"options":{"(gogoproto.castrepeated)":"StringEvents","(gogoproto.nullable)":false}}}},"StringEvent":{"options":{"(gogoproto.stringer)":true},"fields":{"type":{"type":"string","id":1},"attributes":{"rule":"repeated","type":"Attribute","id":2,"options":{"(gogoproto.nullable)":false}}}},"Attribute":{"fields":{"key":{"type":"string","id":1},"value":{"type":"string","id":2}}},"GasInfo":{"fields":{"gasWanted":{"type":"uint64","id":1,"options":{"(gogoproto.moretags)":"yaml:gas_wanted"}},"gasUsed":{"type":"uint64","id":2,"options":{"(gogoproto.moretags)":"yaml:gas_used"}}}},"Result":{"options":{"(gogoproto.goproto_getters)":false},"fields":{"data":{"type":"bytes","id":1},"log":{"type":"string","id":2},"events":{"rule":"repeated","type":"tendermint.abci.Event","id":3,"options":{"(gogoproto.nullable)":false}}}},"SimulationResponse":{"fields":{"gasInfo":{"type":"GasInfo","id":1,"options":{"(gogoproto.embed)":true,"(gogoproto.nullable)":false}},"result":{"type":"Result","id":2}}},"MsgData":{"options":{"(gogoproto.stringer)":true},"fields":{"msgType":{"type":"string","id":1},"data":{"type":"bytes","id":2}}},"TxMsgData":{"options":{"(gogoproto.stringer)":true},"fields":{"data":{"rule":"repeated","type":"MsgData","id":1}}},"SearchTxsResult":{"options":{"(gogoproto.stringer)":true},"fields":{"totalCount":{"type":"uint64","id":1,"options":{"(gogoproto.moretags)":"yaml:total_count","(gogoproto.jsontag)":"total_count"}},"count":{"type":"uint64","id":2},"pageNumber":{"type":"uint64","id":3,"options":{"(gogoproto.moretags)":"yaml:page_number","(gogoproto.jsontag)":"page_number"}},"pageTotal":{"type":"uint64","id":4,"options":{"(gogoproto.moretags)":"yaml:page_total","(gogoproto.jsontag)":"page_total"}},"limit":{"type":"uint64","id":5},"txs":{"rule":"repeated","type":"TxResponse","id":6}}}}}}}}}}},"gogoproto":{"options":{"java_package":"com.google.protobuf","java_outer_classname":"GoGoProtos","go_package":"github.com/gogo/protobuf/gogoproto"},"nested":{"goprotoEnumPrefix":{"type":"bool","id":62001,"extend":"google.protobuf.EnumOptions"},"goprotoEnumStringer":{"type":"bool","id":62021,"extend":"google.protobuf.EnumOptions"},"enumStringer":{"type":"bool","id":62022,"extend":"google.protobuf.EnumOptions"},"enumCustomname":{"type":"string","id":62023,"extend":"google.protobuf.EnumOptions"},"enumdecl":{"type":"bool","id":62024,"extend":"google.protobuf.EnumOptions"},"enumvalueCustomname":{"type":"string","id":66001,"extend":"google.protobuf.EnumValueOptions"},"goprotoGettersAll":{"type":"bool","id":63001,"extend":"google.protobuf.FileOptions"},"goprotoEnumPrefixAll":{"type":"bool","id":63002,"extend":"google.protobuf.FileOptions"},"goprotoStringerAll":{"type":"bool","id":63003,"extend":"google.protobuf.FileOptions"},"verboseEqualAll":{"type":"bool","id":63004,"extend":"google.protobuf.FileOptions"},"faceAll":{"type":"bool","id":63005,"extend":"google.protobuf.FileOptions"},"gostringAll":{"type":"bool","id":63006,"extend":"google.protobuf.FileOptions"},"populateAll":{"type":"bool","id":63007,"extend":"google.protobuf.FileOptions"},"stringerAll":{"type":"bool","id":63008,"extend":"google.protobuf.FileOptions"},"onlyoneAll":{"type":"bool","id":63009,"extend":"google.protobuf.FileOptions"},"equalAll":{"type":"bool","id":63013,"extend":"google.protobuf.FileOptions"},"descriptionAll":{"type":"bool","id":63014,"extend":"google.protobuf.FileOptions"},"testgenAll":{"type":"bool","id":63015,"extend":"google.protobuf.FileOptions"},"benchgenAll":{"type":"bool","id":63016,"extend":"google.protobuf.FileOptions"},"marshalerAll":{"type":"bool","id":63017,"extend":"google.protobuf.FileOptions"},"unmarshalerAll":{"type":"bool","id":63018,"extend":"google.protobuf.FileOptions"},"stableMarshalerAll":{"type":"bool","id":63019,"extend":"google.protobuf.FileOptions"},"sizerAll":{"type":"bool","id":63020,"extend":"google.protobuf.FileOptions"},"goprotoEnumStringerAll":{"type":"bool","id":63021,"extend":"google.protobuf.FileOptions"},"enumStringerAll":{"type":"bool","id":63022,"extend":"google.protobuf.FileOptions"},"unsafeMarshalerAll":{"type":"bool","id":63023,"extend":"google.protobuf.FileOptions"},"unsafeUnmarshalerAll":{"type":"bool","id":63024,"extend":"google.protobuf.FileOptions"},"goprotoExtensionsMapAll":{"type":"bool","id":63025,"extend":"google.protobuf.FileOptions"},"goprotoUnrecognizedAll":{"type":"bool","id":63026,"extend":"google.protobuf.FileOptions"},"gogoprotoImport":{"type":"bool","id":63027,"extend":"google.protobuf.FileOptions"},"protosizerAll":{"type":"bool","id":63028,"extend":"google.protobuf.FileOptions"},"compareAll":{"type":"bool","id":63029,"extend":"google.protobuf.FileOptions"},"typedeclAll":{"type":"bool","id":63030,"extend":"google.protobuf.FileOptions"},"enumdeclAll":{"type":"bool","id":63031,"extend":"google.protobuf.FileOptions"},"goprotoRegistration":{"type":"bool","id":63032,"extend":"google.protobuf.FileOptions"},"messagenameAll":{"type":"bool","id":63033,"extend":"google.protobuf.FileOptions"},"goprotoSizecacheAll":{"type":"bool","id":63034,"extend":"google.protobuf.FileOptions"},"goprotoUnkeyedAll":{"type":"bool","id":63035,"extend":"google.protobuf.FileOptions"},"goprotoGetters":{"type":"bool","id":64001,"extend":"google.protobuf.MessageOptions"},"goprotoStringer":{"type":"bool","id":64003,"extend":"google.protobuf.MessageOptions"},"verboseEqual":{"type":"bool","id":64004,"extend":"google.protobuf.MessageOptions"},"face":{"type":"bool","id":64005,"extend":"google.protobuf.MessageOptions"},"gostring":{"type":"bool","id":64006,"extend":"google.protobuf.MessageOptions"},"populate":{"type":"bool","id":64007,"extend":"google.protobuf.MessageOptions"},"stringer":{"type":"bool","id":67008,"extend":"google.protobuf.MessageOptions"},"onlyone":{"type":"bool","id":64009,"extend":"google.protobuf.MessageOptions"},"equal":{"type":"bool","id":64013,"extend":"google.protobuf.MessageOptions"},"description":{"type":"bool","id":64014,"extend":"google.protobuf.MessageOptions"},"testgen":{"type":"bool","id":64015,"extend":"google.protobuf.MessageOptions"},"benchgen":{"type":"bool","id":64016,"extend":"google.protobuf.MessageOptions"},"marshaler":{"type":"bool","id":64017,"extend":"google.protobuf.MessageOptions"},"unmarshaler":{"type":"bool","id":64018,"extend":"google.protobuf.MessageOptions"},"stableMarshaler":{"type":"bool","id":64019,"extend":"google.protobuf.MessageOptions"},"sizer":{"type":"bool","id":64020,"extend":"google.protobuf.MessageOptions"},"unsafeMarshaler":{"type":"bool","id":64023,"extend":"google.protobuf.MessageOptions"},"unsafeUnmarshaler":{"type":"bool","id":64024,"extend":"google.protobuf.MessageOptions"},"goprotoExtensionsMap":{"type":"bool","id":64025,"extend":"google.protobuf.MessageOptions"},"goprotoUnrecognized":{"type":"bool","id":64026,"extend":"google.protobuf.MessageOptions"},"protosizer":{"type":"bool","id":64028,"extend":"google.protobuf.MessageOptions"},"compare":{"type":"bool","id":64029,"extend":"google.protobuf.MessageOptions"},"typedecl":{"type":"bool","id":64030,"extend":"google.protobuf.MessageOptions"},"messagename":{"type":"bool","id":64033,"extend":"google.protobuf.MessageOptions"},"goprotoSizecache":{"type":"bool","id":64034,"extend":"google.protobuf.MessageOptions"},"goprotoUnkeyed":{"type":"bool","id":64035,"extend":"google.protobuf.MessageOptions"},"nullable":{"type":"bool","id":65001,"extend":"google.protobuf.FieldOptions"},"embed":{"type":"bool","id":65002,"extend":"google.protobuf.FieldOptions"},"customtype":{"type":"string","id":65003,"extend":"google.protobuf.FieldOptions"},"customname":{"type":"string","id":65004,"extend":"google.protobuf.FieldOptions"},"jsontag":{"type":"string","id":65005,"extend":"google.protobuf.FieldOptions"},"moretags":{"type":"string","id":65006,"extend":"google.protobuf.FieldOptions"},"casttype":{"type":"string","id":65007,"extend":"google.protobuf.FieldOptions"},"castkey":{"type":"string","id":65008,"extend":"google.protobuf.FieldOptions"},"castvalue":{"type":"string","id":65009,"extend":"google.protobuf.FieldOptions"},"stdtime":{"type":"bool","id":65010,"extend":"google.protobuf.FieldOptions"},"stdduration":{"type":"bool","id":65011,"extend":"google.protobuf.FieldOptions"},"wktpointer":{"type":"bool","id":65012,"extend":"google.protobuf.FieldOptions"},"castrepeated":{"type":"string","id":65013,"extend":"google.protobuf.FieldOptions"}}},"tendermint":{"nested":{"abci":{"options":{"go_package":"github.com/tendermint/tendermint/abci/types"},"nested":{"Request":{"oneofs":{"value":{"oneof":["echo","flush","info","setOption","initChain","query","beginBlock","checkTx","deliverTx","endBlock","commit","listSnapshots","offerSnapshot","loadSnapshotChunk","applySnapshotChunk"]}},"fields":{"echo":{"type":"RequestEcho","id":1},"flush":{"type":"RequestFlush","id":2},"info":{"type":"RequestInfo","id":3},"setOption":{"type":"RequestSetOption","id":4},"initChain":{"type":"RequestInitChain","id":5},"query":{"type":"RequestQuery","id":6},"beginBlock":{"type":"RequestBeginBlock","id":7},"checkTx":{"type":"RequestCheckTx","id":8},"deliverTx":{"type":"RequestDeliverTx","id":9},"endBlock":{"type":"RequestEndBlock","id":10},"commit":{"type":"RequestCommit","id":11},"listSnapshots":{"type":"RequestListSnapshots","id":12},"offerSnapshot":{"type":"RequestOfferSnapshot","id":13},"loadSnapshotChunk":{"type":"RequestLoadSnapshotChunk","id":14},"applySnapshotChunk":{"type":"RequestApplySnapshotChunk","id":15}}},"RequestEcho":{"fields":{"message":{"type":"string","id":1}}},"RequestFlush":{"fields":{}},"RequestInfo":{"fields":{"version":{"type":"string","id":1},"blockVersion":{"type":"uint64","id":2},"p2pVersion":{"type":"uint64","id":3}}},"RequestSetOption":{"fields":{"key":{"type":"string","id":1},"value":{"type":"string","id":2}}},"RequestInitChain":{"fields":{"time":{"type":"google.protobuf.Timestamp","id":1,"options":{"(gogoproto.nullable)":false,"(gogoproto.stdtime)":true}},"chainId":{"type":"string","id":2},"consensusParams":{"type":"ConsensusParams","id":3},"validators":{"rule":"repeated","type":"ValidatorUpdate","id":4,"options":{"(gogoproto.nullable)":false}},"appStateBytes":{"type":"bytes","id":5},"initialHeight":{"type":"int64","id":6}}},"RequestQuery":{"fields":{"data":{"type":"bytes","id":1},"path":{"type":"string","id":2},"height":{"type":"int64","id":3},"prove":{"type":"bool","id":4}}},"RequestBeginBlock":{"fields":{"hash":{"type":"bytes","id":1},"header":{"type":"tendermint.types.Header","id":2,"options":{"(gogoproto.nullable)":false}},"lastCommitInfo":{"type":"LastCommitInfo","id":3,"options":{"(gogoproto.nullable)":false}},"byzantineValidators":{"rule":"repeated","type":"Evidence","id":4,"options":{"(gogoproto.nullable)":false}}}},"CheckTxType":{"values":{"NEW":0,"RECHECK":1}},"RequestCheckTx":{"fields":{"tx":{"type":"bytes","id":1},"type":{"type":"CheckTxType","id":2}}},"RequestDeliverTx":{"fields":{"tx":{"type":"bytes","id":1}}},"RequestEndBlock":{"fields":{"height":{"type":"int64","id":1}}},"RequestCommit":{"fields":{}},"RequestListSnapshots":{"fields":{}},"RequestOfferSnapshot":{"fields":{"snapshot":{"type":"Snapshot","id":1},"appHash":{"type":"bytes","id":2}}},"RequestLoadSnapshotChunk":{"fields":{"height":{"type":"uint64","id":1},"format":{"type":"uint32","id":2},"chunk":{"type":"uint32","id":3}}},"RequestApplySnapshotChunk":{"fields":{"index":{"type":"uint32","id":1},"chunk":{"type":"bytes","id":2},"sender":{"type":"string","id":3}}},"Response":{"oneofs":{"value":{"oneof":["exception","echo","flush","info","setOption","initChain","query","beginBlock","checkTx","deliverTx","endBlock","commit","listSnapshots","offerSnapshot","loadSnapshotChunk","applySnapshotChunk"]}},"fields":{"exception":{"type":"ResponseException","id":1},"echo":{"type":"ResponseEcho","id":2},"flush":{"type":"ResponseFlush","id":3},"info":{"type":"ResponseInfo","id":4},"setOption":{"type":"ResponseSetOption","id":5},"initChain":{"type":"ResponseInitChain","id":6},"query":{"type":"ResponseQuery","id":7},"beginBlock":{"type":"ResponseBeginBlock","id":8},"checkTx":{"type":"ResponseCheckTx","id":9},"deliverTx":{"type":"ResponseDeliverTx","id":10},"endBlock":{"type":"ResponseEndBlock","id":11},"commit":{"type":"ResponseCommit","id":12},"listSnapshots":{"type":"ResponseListSnapshots","id":13},"offerSnapshot":{"type":"ResponseOfferSnapshot","id":14},"loadSnapshotChunk":{"type":"ResponseLoadSnapshotChunk","id":15},"applySnapshotChunk":{"type":"ResponseApplySnapshotChunk","id":16}}},"ResponseException":{"fields":{"error":{"type":"string","id":1}}},"ResponseEcho":{"fields":{"message":{"type":"string","id":1}}},"ResponseFlush":{"fields":{}},"ResponseInfo":{"fields":{"data":{"type":"string","id":1},"version":{"type":"string","id":2},"appVersion":{"type":"uint64","id":3},"lastBlockHeight":{"type":"int64","id":4},"lastBlockAppHash":{"type":"bytes","id":5}}},"ResponseSetOption":{"fields":{"code":{"type":"uint32","id":1},"log":{"type":"string","id":3},"info":{"type":"string","id":4}}},"ResponseInitChain":{"fields":{"consensusParams":{"type":"ConsensusParams","id":1},"validators":{"rule":"repeated","type":"ValidatorUpdate","id":2,"options":{"(gogoproto.nullable)":false}},"appHash":{"type":"bytes","id":3}}},"ResponseQuery":{"fields":{"code":{"type":"uint32","id":1},"log":{"type":"string","id":3},"info":{"type":"string","id":4},"index":{"type":"int64","id":5},"key":{"type":"bytes","id":6},"value":{"type":"bytes","id":7},"proofOps":{"type":"tendermint.crypto.ProofOps","id":8},"height":{"type":"int64","id":9},"codespace":{"type":"string","id":10}}},"ResponseBeginBlock":{"fields":{"events":{"rule":"repeated","type":"Event","id":1,"options":{"(gogoproto.nullable)":false,"(gogoproto.jsontag)":"events,omitempty"}}}},"ResponseCheckTx":{"fields":{"code":{"type":"uint32","id":1},"data":{"type":"bytes","id":2},"log":{"type":"string","id":3},"info":{"type":"string","id":4},"gasWanted":{"type":"int64","id":5,"options":{"json_name":"gas_wanted"}},"gasUsed":{"type":"int64","id":6,"options":{"json_name":"gas_used"}},"events":{"rule":"repeated","type":"Event","id":7,"options":{"(gogoproto.nullable)":false,"(gogoproto.jsontag)":"events,omitempty"}},"codespace":{"type":"string","id":8}}},"ResponseDeliverTx":{"fields":{"code":{"type":"uint32","id":1},"data":{"type":"bytes","id":2},"log":{"type":"string","id":3},"info":{"type":"string","id":4},"gasWanted":{"type":"int64","id":5,"options":{"json_name":"gas_wanted"}},"gasUsed":{"type":"int64","id":6,"options":{"json_name":"gas_used"}},"events":{"rule":"repeated","type":"Event","id":7,"options":{"(gogoproto.nullable)":false,"(gogoproto.jsontag)":"events,omitempty"}},"codespace":{"type":"string","id":8}}},"ResponseEndBlock":{"fields":{"validatorUpdates":{"rule":"repeated","type":"ValidatorUpdate","id":1,"options":{"(gogoproto.nullable)":false}},"consensusParamUpdates":{"type":"ConsensusParams","id":2},"events":{"rule":"repeated","type":"Event","id":3,"options":{"(gogoproto.nullable)":false,"(gogoproto.jsontag)":"events,omitempty"}}}},"ResponseCommit":{"fields":{"data":{"type":"bytes","id":2},"retainHeight":{"type":"int64","id":3}}},"ResponseListSnapshots":{"fields":{"snapshots":{"rule":"repeated","type":"Snapshot","id":1}}},"ResponseOfferSnapshot":{"fields":{"result":{"type":"Result","id":1}},"nested":{"Result":{"values":{"UNKNOWN":0,"ACCEPT":1,"ABORT":2,"REJECT":3,"REJECT_FORMAT":4,"REJECT_SENDER":5}}}},"ResponseLoadSnapshotChunk":{"fields":{"chunk":{"type":"bytes","id":1}}},"ResponseApplySnapshotChunk":{"fields":{"result":{"type":"Result","id":1},"refetchChunks":{"rule":"repeated","type":"uint32","id":2},"rejectSenders":{"rule":"repeated","type":"string","id":3}},"nested":{"Result":{"values":{"UNKNOWN":0,"ACCEPT":1,"ABORT":2,"RETRY":3,"RETRY_SNAPSHOT":4,"REJECT_SNAPSHOT":5}}}},"ConsensusParams":{"fields":{"block":{"type":"BlockParams","id":1},"evidence":{"type":"tendermint.types.EvidenceParams","id":2},"validator":{"type":"tendermint.types.ValidatorParams","id":3},"version":{"type":"tendermint.types.VersionParams","id":4}}},"BlockParams":{"fields":{"maxBytes":{"type":"int64","id":1},"maxGas":{"type":"int64","id":2}}},"LastCommitInfo":{"fields":{"round":{"type":"int32","id":1},"votes":{"rule":"repeated","type":"VoteInfo","id":2,"options":{"(gogoproto.nullable)":false}}}},"Event":{"fields":{"type":{"type":"string","id":1},"attributes":{"rule":"repeated","type":"EventAttribute","id":2,"options":{"(gogoproto.nullable)":false,"(gogoproto.jsontag)":"attributes,omitempty"}}}},"EventAttribute":{"fields":{"key":{"type":"bytes","id":1},"value":{"type":"bytes","id":2},"index":{"type":"bool","id":3}}},"TxResult":{"fields":{"height":{"type":"int64","id":1},"index":{"type":"uint32","id":2},"tx":{"type":"bytes","id":3},"result":{"type":"ResponseDeliverTx","id":4,"options":{"(gogoproto.nullable)":false}}}},"Validator":{"fields":{"address":{"type":"bytes","id":1},"power":{"type":"int64","id":3}}},"ValidatorUpdate":{"fields":{"pubKey":{"type":"tendermint.crypto.PublicKey","id":1,"options":{"(gogoproto.nullable)":false}},"power":{"type":"int64","id":2}}},"VoteInfo":{"fields":{"validator":{"type":"Validator","id":1,"options":{"(gogoproto.nullable)":false}},"signedLastBlock":{"type":"bool","id":2}}},"EvidenceType":{"values":{"UNKNOWN":0,"DUPLICATE_VOTE":1,"LIGHT_CLIENT_ATTACK":2}},"Evidence":{"fields":{"type":{"type":"EvidenceType","id":1},"validator":{"type":"Validator","id":2,"options":{"(gogoproto.nullable)":false}},"height":{"type":"int64","id":3},"time":{"type":"google.protobuf.Timestamp","id":4,"options":{"(gogoproto.nullable)":false,"(gogoproto.stdtime)":true}},"totalVotingPower":{"type":"int64","id":5}}},"Snapshot":{"fields":{"height":{"type":"uint64","id":1},"format":{"type":"uint32","id":2},"chunks":{"type":"uint32","id":3},"hash":{"type":"bytes","id":4},"metadata":{"type":"bytes","id":5}}},"ABCIApplication":{"methods":{"Echo":{"requestType":"RequestEcho","responseType":"ResponseEcho"},"Flush":{"requestType":"RequestFlush","responseType":"ResponseFlush"},"Info":{"requestType":"RequestInfo","responseType":"ResponseInfo"},"SetOption":{"requestType":"RequestSetOption","responseType":"ResponseSetOption"},"DeliverTx":{"requestType":"RequestDeliverTx","responseType":"ResponseDeliverTx"},"CheckTx":{"requestType":"RequestCheckTx","responseType":"ResponseCheckTx"},"Query":{"requestType":"RequestQuery","responseType":"ResponseQuery"},"Commit":{"requestType":"RequestCommit","responseType":"ResponseCommit"},"InitChain":{"requestType":"RequestInitChain","responseType":"ResponseInitChain"},"BeginBlock":{"requestType":"RequestBeginBlock","responseType":"ResponseBeginBlock"},"EndBlock":{"requestType":"RequestEndBlock","responseType":"ResponseEndBlock"},"ListSnapshots":{"requestType":"RequestListSnapshots","responseType":"ResponseListSnapshots"},"OfferSnapshot":{"requestType":"RequestOfferSnapshot","responseType":"ResponseOfferSnapshot"},"LoadSnapshotChunk":{"requestType":"RequestLoadSnapshotChunk","responseType":"ResponseLoadSnapshotChunk"},"ApplySnapshotChunk":{"requestType":"RequestApplySnapshotChunk","responseType":"ResponseApplySnapshotChunk"}}}}}}},"google":{"nested":{"protobuf":{"nested":{"Timestamp":{"fields":{"seconds":{"type":"int64","id":1},"nanos":{"type":"int32","id":2}}}}}}}}}';

// todo: dynamically create this type from protobuf
export class MsgData {
  msgType: string;
  data: string | undefined;

  constructor(msgType: string, data: string | undefined) {
    this.msgType = msgType;
    this.data = data;
  }
}

export function decodeTxData(data: Uint8Array): MsgData[] {
  // const root = protobuf.loadSync(
  //   ["/mnt/c/Users/Bob/GolandProjects/SecretNetwork/third_party/proto/cosmos/base/abci/v1beta1/abci.proto",
  //   "/mnt/c/Users/Bob/GolandProjects/SecretNetwork/third_party/proto/gogoproto/gogo.proto",
  //   "/mnt/c/Users/Bob/GolandProjects/SecretNetwork/third_party/proto/tendermint/abci/types.proto"]
  // );

  const root = protobuf.Root.fromJSON(JSON.parse(descriptor));

  let TxMsgData = root.lookupType("cosmos.base.abci.v1beta1.TxMsgData");

  let message = TxMsgData.decode(data);

  let object = TxMsgData.toObject(message, {
    longs: String,
    enums: String,
    bytes: String,
    // see ConversionOptions
  });

  // console.log(`proto decrypt response: ${JSON.stringify(object)}`);

  return object["data"].map((item: any) => {
    return new MsgData(item.msgType, item?.data);
  });
}
