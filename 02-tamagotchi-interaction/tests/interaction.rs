use gtest::{Program, System, Log};
use tmg2_io::{Tamagotchi, TmgAction, TmgEvent};

// Yury: this is just a basic test
// I didn't find if it's possible to manually advance blocks in the mock environment for testing Mood changes
// The printouts always return default values, regardless if they are previously initialized, it's unclear to why.
// However I tested the contract functionality on the testnet by sending relevant messages manually.
#[test]
fn interaction_test() {
    let sys = System::new();
    sys.init_logger();
    let program = Program::current(&sys);

    let res = program.send(2, "Tama".to_string());
    assert!(!res.main_failed());

    let res = program.send(2, TmgAction::Name);
    
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Name("????".to_string()));
    assert!(res.contains(&expected_log));

    let name = res.decoded_log::<String>();
    println!("decoded_reply: {:#?}", name);

    let res = program.send(2, TmgAction::Age); 
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Age(5));
    assert!(res.contains(&expected_log));
    let age = res.decoded_log::<u64>();
    println!("age: {:#?} ms", age);

    let res = program.send(2, TmgAction::Feed); 
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Fed);
    assert!(res.contains(&expected_log));

    let res = program.send(2, TmgAction::Entertain); 
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Entertained);
    assert!(res.contains(&expected_log));
    
    let res = program.send(2, TmgAction::Sleep); 
    let expected_log = Log::builder()
        .dest(2)
        .payload(TmgEvent::Slept);
    assert!(res.contains(&expected_log));

    let res = program.send(2, TmgAction::Full); 
    let tmg = res.decoded_log::<Tamagotchi>();
    println!("FULL: {:#?}", tmg);


}
