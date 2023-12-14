/*
 * Created using https://github.com/a2x/cs2-dumper
 * Thu, 14 Dec 2023 23:21:54 +0000
 */

#![allow(non_snake_case, non_upper_case_globals)]

pub mod CBasePulseGraphInstance {
}

pub mod CPulseCell_Base {
    pub const m_nEditorNodeID: usize = 0x8; // PulseDocNodeID_t
}

pub mod CPulseCell_BaseFlow { // CPulseCell_Base
}

pub mod CPulseCell_BaseValue { // CPulseCell_Base
}

pub mod CPulseCell_BaseYieldingInflow { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Inflow_BaseEntrypoint { // CPulseCell_BaseFlow
    pub const m_EntryChunk: usize = 0x48; // PulseRuntimeChunkIndex_t
    pub const m_RegisterMap: usize = 0x50; // PulseRegisterMap_t
}

pub mod CPulseCell_Inflow_EntOutputHandler { // CPulseCell_Inflow_BaseEntrypoint
    pub const m_SourceEntity: usize = 0x70; // CUtlSymbolLarge
    pub const m_SourceOutput: usize = 0x78; // CUtlSymbolLarge
    pub const m_TargetInput: usize = 0x80; // CUtlSymbolLarge
    pub const m_ExpectedParamType: usize = 0x88; // CPulseValueFullType
}

pub mod CPulseCell_Inflow_EventHandler { // CPulseCell_Inflow_BaseEntrypoint
    pub const m_EventName: usize = 0x70; // CUtlSymbolLarge
}

pub mod CPulseCell_Inflow_GraphHook { // CPulseCell_Inflow_BaseEntrypoint
    pub const m_HookName: usize = 0x70; // CUtlSymbolLarge
}

pub mod CPulseCell_Inflow_Method { // CPulseCell_Inflow_BaseEntrypoint
    pub const m_MethodName: usize = 0x70; // CUtlSymbolLarge
    pub const m_Description: usize = 0x78; // CUtlString
    pub const m_bIsPublic: usize = 0x80; // bool
    pub const m_ReturnType: usize = 0x88; // CPulseValueFullType
    pub const m_Args: usize = 0x98; // CUtlVector<CPulseRuntimeMethodArg>
}

pub mod CPulseCell_Inflow_Wait { // CPulseCell_BaseYieldingInflow
    pub const m_WakeResume: usize = 0x48; // CPulse_ResumePoint
}

pub mod CPulseCell_Inflow_Yield { // CPulseCell_BaseYieldingInflow
    pub const m_UnyieldResume: usize = 0x48; // CPulse_ResumePoint
}

pub mod CPulseCell_Outflow_CycleOrdered { // CPulseCell_BaseFlow
    pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
}

pub mod CPulseCell_Outflow_CycleOrdered_InstanceState_t {
    pub const m_nNextIndex: usize = 0x0; // int32_t
}

pub mod CPulseCell_Outflow_CycleRandom { // CPulseCell_BaseFlow
    pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
}

pub mod CPulseCell_Outflow_CycleShuffled { // CPulseCell_BaseFlow
    pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
}

pub mod CPulseCell_Outflow_CycleShuffled_InstanceState_t {
    pub const m_Shuffle: usize = 0x0; // CUtlVectorFixedGrowable<uint8_t>
    pub const m_nNextShuffle: usize = 0x20; // int32_t
}

pub mod CPulseCell_Outflow_IntSwitch { // CPulseCell_BaseFlow
    pub const m_DefaultCaseOutflow: usize = 0x48; // CPulse_OutflowConnection
    pub const m_CaseOutflows: usize = 0x58; // CUtlVector<CPulse_OutflowConnection>
}

pub mod CPulseCell_Outflow_SimultaneousParallel { // CPulseCell_BaseFlow
    pub const m_Outputs: usize = 0x48; // CUtlVector<CPulse_OutflowConnection>
}

pub mod CPulseCell_Outflow_StringSwitch { // CPulseCell_BaseFlow
    pub const m_DefaultCaseOutflow: usize = 0x48; // CPulse_OutflowConnection
    pub const m_CaseOutflows: usize = 0x58; // CUtlVector<CPulse_OutflowConnection>
}

pub mod CPulseCell_Outflow_TestExplicitYesNo { // CPulseCell_BaseFlow
    pub const m_Yes: usize = 0x48; // CPulse_OutflowConnection
    pub const m_No: usize = 0x58; // CPulse_OutflowConnection
}

pub mod CPulseCell_Outflow_TestRandomYesNo { // CPulseCell_BaseFlow
    pub const m_Yes: usize = 0x48; // CPulse_OutflowConnection
    pub const m_No: usize = 0x58; // CPulse_OutflowConnection
}

pub mod CPulseCell_Step_CallExternalMethod { // CPulseCell_BaseFlow
    pub const m_MethodName: usize = 0x48; // CUtlSymbolLarge
    pub const m_ExpectedArgs: usize = 0x50; // CUtlVector<CPulseRuntimeMethodArg>
}

pub mod CPulseCell_Step_DebugLog { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Step_PublicOutput { // CPulseCell_BaseFlow
    pub const m_OutputIndex: usize = 0x48; // PulseRuntimeOutputIndex_t
}

pub mod CPulseCell_Step_TestDomainCreateFakeEntity { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Step_TestDomainDestroyFakeEntity { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Step_TestDomainEntFire { // CPulseCell_BaseFlow
    pub const m_Input: usize = 0x48; // CUtlString
}

pub mod CPulseCell_Step_TestDomainTracepoint { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Test_MultiInflow_NoDefault { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Test_MultiInflow_WithDefault { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Test_NoInflow { // CPulseCell_BaseFlow
}

pub mod CPulseCell_Val_TestDomainFindEntityByName { // CPulseCell_BaseValue
}

pub mod CPulseCell_Val_TestDomainGetEntityName { // CPulseCell_BaseValue
}

pub mod CPulseCell_Value_RandomInt { // CPulseCell_BaseValue
}

pub mod CPulseCell_Value_TestValue50 { // CPulseCell_BaseValue
}

pub mod CPulseExecCursor {
}

pub mod CPulseGraphDef {
    pub const m_DomainIdentifier: usize = 0x8; // CUtlSymbolLarge
    pub const m_ParentMapName: usize = 0x10; // CUtlSymbolLarge
    pub const m_Chunks: usize = 0x18; // CUtlVector<CPulse_Chunk*>
    pub const m_Cells: usize = 0x30; // CUtlVector<CPulseCell_Base*>
    pub const m_Vars: usize = 0x48; // CUtlVector<CPulse_Variable>
    pub const m_PublicOutputs: usize = 0x60; // CUtlVector<CPulse_PublicOutput>
    pub const m_InvokeBindings: usize = 0x78; // CUtlVector<CPulse_InvokeBinding*>
    pub const m_CallInfos: usize = 0x90; // CUtlVector<CPulse_CallInfo*>
    pub const m_OutputConnections: usize = 0xA8; // CUtlVector<CPulse_OutputConnection*>
}

pub mod CPulseGraphInstance_TestDomain { // CBasePulseGraphInstance
    pub const m_bIsRunningUnitTests: usize = 0xD0; // bool
    pub const m_bExplicitTimeStepping: usize = 0xD1; // bool
    pub const m_bExpectingToDestroyWithYieldedCursors: usize = 0xD2; // bool
    pub const m_nNextValidateIndex: usize = 0xD4; // int32_t
    pub const m_Tracepoints: usize = 0xD8; // CUtlVector<CUtlString>
    pub const m_bTestYesOrNoPath: usize = 0xF0; // bool
}

pub mod CPulseGraphInstance_TestDomain_Derived { // CPulseGraphInstance_TestDomain
    pub const m_nInstanceValueX: usize = 0xF8; // int32_t
}

pub mod CPulseGraphInstance_TurtleGraphics { // CBasePulseGraphInstance
}

pub mod CPulseMathlib {
}

pub mod CPulseRuntimeMethodArg {
    pub const m_Name: usize = 0x0; // CKV3MemberNameWithStorage
    pub const m_Description: usize = 0x38; // CUtlString
    pub const m_Type: usize = 0x40; // CPulseValueFullType
}

pub mod CPulseTestFuncs_DerivedDomain {
}

pub mod CPulseTestFuncs_LibraryA {
}

pub mod CPulseTestScriptLib {
}

pub mod CPulseTurtleGraphicsCursor { // CPulseExecCursor
    pub const m_Color: usize = 0x188; // Color
    pub const m_vPos: usize = 0x18C; // Vector2D
    pub const m_flHeadingDeg: usize = 0x194; // float
    pub const m_bPenUp: usize = 0x198; // bool
}

pub mod CPulse_CallInfo {
    pub const m_PortName: usize = 0x0; // CUtlSymbolLarge
    pub const m_nEditorNodeID: usize = 0x8; // PulseDocNodeID_t
    pub const m_RegisterMap: usize = 0x10; // PulseRegisterMap_t
    pub const m_CallMethodID: usize = 0x30; // PulseDocNodeID_t
    pub const m_nSrcChunk: usize = 0x34; // PulseRuntimeChunkIndex_t
    pub const m_nSrcInstruction: usize = 0x38; // int32_t
}

pub mod CPulse_Chunk {
    pub const m_Instructions: usize = 0x0; // CUtlLeanVector<PGDInstruction_t>
    pub const m_Registers: usize = 0x10; // CUtlLeanVector<CPulse_RegisterInfo>
    pub const m_InstructionEditorIDs: usize = 0x20; // CUtlLeanVector<PulseDocNodeID_t>
}

pub mod CPulse_InvokeBinding {
    pub const m_RegisterMap: usize = 0x0; // PulseRegisterMap_t
    pub const m_FuncName: usize = 0x20; // CUtlSymbolLarge
    pub const m_nCellIndex: usize = 0x28; // PulseRuntimeCellIndex_t
    pub const m_InstanceType: usize = 0x30; // CPulseValueFullType
    pub const m_nSrcChunk: usize = 0x40; // PulseRuntimeChunkIndex_t
    pub const m_nSrcInstruction: usize = 0x44; // int32_t
}

pub mod CPulse_OutflowConnection {
    pub const m_SourceOutflowName: usize = 0x0; // CUtlSymbolLarge
    pub const m_nDestChunk: usize = 0x8; // PulseRuntimeChunkIndex_t
    pub const m_nInstruction: usize = 0xC; // int32_t
}

pub mod CPulse_OutputConnection {
    pub const m_SourceOutput: usize = 0x0; // CUtlSymbolLarge
    pub const m_TargetEntity: usize = 0x8; // CUtlSymbolLarge
    pub const m_TargetInput: usize = 0x10; // CUtlSymbolLarge
    pub const m_Param: usize = 0x18; // CUtlSymbolLarge
}

pub mod CPulse_PublicOutput {
    pub const m_Name: usize = 0x0; // CUtlSymbolLarge
    pub const m_Description: usize = 0x8; // CUtlString
    pub const m_ParamType: usize = 0x10; // CPulseValueFullType
}

pub mod CPulse_RegisterInfo {
    pub const m_nReg: usize = 0x0; // PulseRuntimeRegisterIndex_t
    pub const m_Type: usize = 0x8; // CPulseValueFullType
    pub const m_OriginName: usize = 0x18; // CKV3MemberNameWithStorage
    pub const m_nWrittenByInstruction: usize = 0x50; // int32_t
    pub const m_nLastReadByInstruction: usize = 0x54; // int32_t
}

pub mod CPulse_ResumePoint { // CPulse_OutflowConnection
}

pub mod CPulse_Variable {
    pub const m_Name: usize = 0x0; // CUtlSymbolLarge
    pub const m_Description: usize = 0x8; // CUtlString
    pub const m_Type: usize = 0x10; // CPulseValueFullType
    pub const m_DefaultValue: usize = 0x20; // KeyValues3
    pub const m_bIsPublic: usize = 0x32; // bool
}

pub mod CTestDomainDerived_Cursor { // CPulseExecCursor
    pub const m_nCursorValueA: usize = 0x188; // int32_t
    pub const m_nCursorValueB: usize = 0x18C; // int32_t
}

pub mod FakeEntity_t {
    pub const m_nHandle: usize = 0x0; // PulseTestEHandle_t
    pub const m_Name: usize = 0x8; // CUtlString
    pub const m_Class: usize = 0x10; // CUtlString
    pub const m_bDestroyed: usize = 0x18; // bool
    pub const m_pAssociatedGraphInstance: usize = 0x20; // CPulseGraphInstance_TestDomain*
    pub const m_bFuncWasCalled: usize = 0x28; // bool
    pub const m_fValue: usize = 0x2C; // float
}

pub mod PGDInstruction_t {
    pub const m_nCode: usize = 0x0; // PulseInstructionCode_t
    pub const m_nVar: usize = 0x4; // PulseRuntimeVarIndex_t
    pub const m_nReg0: usize = 0x8; // PulseRuntimeRegisterIndex_t
    pub const m_nReg1: usize = 0xA; // PulseRuntimeRegisterIndex_t
    pub const m_nReg2: usize = 0xC; // PulseRuntimeRegisterIndex_t
    pub const m_nInvokeBindingIndex: usize = 0x10; // PulseRuntimeInvokeIndex_t
    pub const m_nChunk: usize = 0x14; // PulseRuntimeChunkIndex_t
    pub const m_nDestInstruction: usize = 0x18; // int32_t
    pub const m_nCallInfoIndex: usize = 0x1C; // PulseRuntimeCallInfoIndex_t
    pub const m_Arg0Name: usize = 0x20; // CUtlSymbolLarge
    pub const m_Arg1Name: usize = 0x28; // CUtlSymbolLarge
    pub const m_bLiteralBool: usize = 0x30; // bool
    pub const m_nLiteralInt: usize = 0x34; // int32_t
    pub const m_flLiteralFloat: usize = 0x38; // float
    pub const m_LiteralString: usize = 0x40; // CBufferString
    pub const m_vLiteralVec3: usize = 0x50; // Vector
}

pub mod PulseDocNodeID_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseRegisterMap_t {
    pub const m_Inparams: usize = 0x0; // KeyValues3
    pub const m_Outparams: usize = 0x10; // KeyValues3
}

pub mod PulseRuntimeCallInfoIndex_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseRuntimeCellIndex_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseRuntimeChunkIndex_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseRuntimeEntrypointIndex_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseRuntimeInvokeIndex_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseRuntimeOutputIndex_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseRuntimeRegisterIndex_t {
    pub const m_Value: usize = 0x0; // int16_t
}

pub mod PulseRuntimeStateOffset_t {
    pub const m_Value: usize = 0x0; // uint16_t
}

pub mod PulseRuntimeVarIndex_t {
    pub const m_Value: usize = 0x0; // int32_t
}

pub mod PulseTestEHandle_t {
    pub const m_Value: usize = 0x0; // int32_t
}