fn main() {
    protobuf_codegen::Codegen::new()
        .pure()
        .include("proto")
        .inputs(&[
            "proto/InitConnect.proto",
            "proto/Common.proto",
            "proto/Qot_Common.proto",
            "proto/Qot_GetIpoList.proto",
            "proto/Qot_GetSecuritySnapshot.proto",
            "proto/Qot_GetUserSecurityGroup.proto",
            "proto/Qot_GetUserSecurity.proto",
            "proto/Qot_ModifyUserSecurity.proto",
            "proto/Qot_GetPlateSecurity.proto",
            "proto/Qot_StockFilter.proto",
            "proto/Qot_Sub.proto",
            "proto/Qot_GetBasicQot.proto",
            "proto/Qot_UpdateBasicQot.proto",
            "proto/GetGlobalState.proto",
            "proto/Qot_GetPriceReminder.proto",
            "proto/Qot_SetPriceReminder.proto",
            "proto/Trd_Common.proto",
            "proto/Trd_UnlockTrade.proto",
            "proto/Trd_PlaceOrder.proto",
            "proto/Trd_GetMaxTrdQtys.proto",
            "proto/Trd_GetPositionList.proto",
            "proto/Trd_GetHistoryOrderList.proto",
            "proto/Trd_ModifyOrder.proto",
            "proto/KeepAlive.proto",
            "proto/Qot_UpdateRT.proto",
            "proto/Qot_UpdateKL.proto",
        ])
        .cargo_out_dir("rust_protobuf_protos")
        .run_from_script();
}
