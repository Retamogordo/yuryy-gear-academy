use gtest::{Program, System, Log};
use tmg3_io::{TmgAction, TmgEvent};

#[test]
fn owning_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(2, "Tama".to_string());
    assert!(!res.main_failed());
    // transfer ownership to 3
    let res = program.send(2, TmgAction::Transfer(3.into()));
    
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Transferred);

    assert!(res.contains(&expected_log));
    // ensure that 3 holds ownership by re-trasferring ownership to 4
    let res = program.send(3, TmgAction::Transfer(4.into()));
    
    let expected_log = Log::builder()
        .dest(3)
        .payload(TmgEvent::Transferred);

    assert!(res.contains(&expected_log));
}

#[test]
fn fail_transfer_from_non_owning_account_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(2, "Tama".to_string());
    assert!(!res.main_failed());

    let res = program.send(4, TmgAction::Transfer(3.into()));
    assert!(res.main_failed());
}

#[test]
fn approving_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(2, "Tama".to_string());
    assert!(!res.main_failed());
    // approve account 4
    let res = program.send(2, TmgAction::Approve(4.into()));
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Approved);
    assert!(res.contains(&expected_log));

    // transfer ownership to 3
    let res = program.send(2, TmgAction::Transfer(3.into()));
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Transferred);
    assert!(res.contains(&expected_log));
    // 3 revokes approval granted by 2 to 4
    let res = program.send(3, TmgAction::RevokeApproval);
    let expected_log = Log::builder()
        .dest(3)
        .payload(TmgEvent::ApprovalRevoked);
    assert!(res.contains(&expected_log));
}