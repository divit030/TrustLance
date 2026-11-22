
use anchor_lang::prelude::*;
use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas};
use anchor_lang::solana_program::instruction::Instruction;
use litesvm::LiteSVM;
use solana_keypair::Keypair;
use solana_signer::Signer;
use solana_transaction::Transaction;

use project::state::booking::{
    Booking,
    BOOKING_CREATED,
    BOOKING_ACCEPTED,
     BOOKING_COMPLETED,
      BOOKING_APPROVED,
       BOOKING_CANCELLED,
};
use project::state::customer::Customer;
use project::state::escrow::Escrow;
use project::state::provider::Provider;

#[test]
fn test_register_provider() {
    let mut svm = LiteSVM::new();

    let provider_wallet = Keypair::new();

    svm.airdrop(&provider_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (provider_pda, _) = Pubkey::find_program_address(
        &[b"provider", provider_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let accounts = project::accounts::RegisterProvider {
        authority: provider_wallet.pubkey(),
        provider: provider_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let instruction = Instruction {
        program_id: project::ID,
        accounts: accounts.to_account_metas(None),
        data: project::instruction::RegisterProvider {
            name: "Test Provider".to_string(),
            service: "Plumbing".to_string(),
            price_lamports: 500_000_000,
        }
        .data(),
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(transaction).unwrap();

    let provider_account = svm.get_account(&provider_pda).unwrap();

    let provider = Provider::try_deserialize(
        &mut provider_account.data.as_slice(),
    )
    .unwrap();

    assert_eq!(provider.authority, provider_wallet.pubkey());
    assert_eq!(provider.name, "Test Provider");
    assert_eq!(provider.service, "Plumbing");
    assert_eq!(provider.price_lamports, 500_000_000);
    assert!(provider.registered);
}

#[test]
fn test_register_customer() {
    let mut svm = LiteSVM::new();

    let customer_wallet = Keypair::new();

    svm.airdrop(&customer_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (customer_pda, _) = Pubkey::find_program_address(
        &[b"customer", customer_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let accounts = project::accounts::RegisterCustomer {
        authority: customer_wallet.pubkey(),
        customer: customer_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let instruction = Instruction {
        program_id: project::ID,
        accounts: accounts.to_account_metas(None),
        data: project::instruction::RegisterCustomer {
            name: "Test Customer".to_string(),
        }
        .data(),
    };

    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(transaction).unwrap();

    let customer_account = svm.get_account(&customer_pda).unwrap();

    let customer = Customer::try_deserialize(
        &mut customer_account.data.as_slice(),
    )
    .unwrap();

    assert_eq!(customer.authority, customer_wallet.pubkey());
    assert_eq!(customer.name, "Test Customer");
    assert!(customer.registered);
}

#[test]
fn test_create_booking_and_deposit_escrow() {
    let mut svm = LiteSVM::new();

    let provider_wallet = Keypair::new();
    let customer_wallet = Keypair::new();

    svm.airdrop(&provider_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.airdrop(&customer_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (provider_pda, _) = Pubkey::find_program_address(
        &[b"provider", provider_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let (customer_pda, _) = Pubkey::find_program_address(
        &[b"customer", customer_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let register_provider_accounts = project::accounts::RegisterProvider {
        authority: provider_wallet.pubkey(),
        provider: provider_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_provider_instruction = Instruction {
        program_id: project::ID,
        accounts: register_provider_accounts.to_account_metas(None),
        data: project::instruction::RegisterProvider {
            name: "Test Provider".to_string(),
            service: "Plumbing".to_string(),
            price_lamports: 500_000_000,
        }
        .data(),
    };

    let register_provider_transaction = Transaction::new_signed_with_payer(
        &[register_provider_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_provider_transaction).unwrap();

    let register_customer_accounts = project::accounts::RegisterCustomer {
        authority: customer_wallet.pubkey(),
        customer: customer_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_customer_instruction = Instruction {
        program_id: project::ID,
        accounts: register_customer_accounts.to_account_metas(None),
        data: project::instruction::RegisterCustomer {
            name: "Test Customer".to_string(),
        }
        .data(),
    };

    let register_customer_transaction = Transaction::new_signed_with_payer(
        &[register_customer_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_customer_transaction).unwrap();

    let booking_id = 1u64;
    let expected_amount = 500_000_000u64;

    let (booking_pda, _) = Pubkey::find_program_address(
        &[
            b"booking",
            customer_wallet.pubkey().as_ref(),
            booking_id.to_le_bytes().as_ref(),
        ],
        &project::ID,
    );

    let (escrow_pda, _) = Pubkey::find_program_address(
        &[b"escrow", booking_pda.as_ref()],
        &project::ID,
    );

    let provider_balance_before = svm
        .get_account(&provider_wallet.pubkey())
        .unwrap()
        .lamports;

    let create_booking_accounts = project::accounts::CreateBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
        escrow: escrow_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let create_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: create_booking_accounts.to_account_metas(None),
        data: project::instruction::CreateBooking {
            booking_id,
        }
        .data(),
    };

    let create_booking_transaction = Transaction::new_signed_with_payer(
        &[create_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_booking_transaction).unwrap();

    let booking_account = svm.get_account(&booking_pda).unwrap();

    let booking = Booking::try_deserialize(
        &mut booking_account.data.as_slice(),
    )
    .unwrap();

    let escrow_account = svm.get_account(&escrow_pda).unwrap();

    let escrow = Escrow::try_deserialize(
        &mut escrow_account.data.as_slice(),
    )
    .unwrap();

    let provider_balance_after = svm
        .get_account(&provider_wallet.pubkey())
        .unwrap()
        .lamports;

    assert_eq!(booking.booking_id, booking_id);
    assert_eq!(booking.customer, customer_wallet.pubkey());
    assert_eq!(booking.provider, provider_wallet.pubkey());
    assert_eq!(booking.service, "Plumbing");
    assert_eq!(booking.amount_lamports, expected_amount);
    assert_eq!(booking.status, BOOKING_CREATED);

    assert_eq!(escrow.booking, booking_pda);
    assert_eq!(escrow.customer, customer_wallet.pubkey());
    assert_eq!(escrow.provider, provider_wallet.pubkey());
    assert_eq!(escrow.amount_lamports, expected_amount);

    assert!(escrow_account.lamports >= expected_amount);
    assert_eq!(provider_balance_before, provider_balance_after);
}


#[test]
fn test_accept_booking() {
    let mut svm = LiteSVM::new();

    let provider_wallet = Keypair::new();
    let customer_wallet = Keypair::new();

    svm.airdrop(&provider_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.airdrop(&customer_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (provider_pda, _) = Pubkey::find_program_address(
        &[b"provider", provider_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let (customer_pda, _) = Pubkey::find_program_address(
        &[b"customer", customer_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let register_provider_accounts = project::accounts::RegisterProvider {
        authority: provider_wallet.pubkey(),
        provider: provider_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_provider_instruction = Instruction {
        program_id: project::ID,
        accounts: register_provider_accounts.to_account_metas(None),
        data: project::instruction::RegisterProvider {
            name: "Test Provider".to_string(),
            service: "Plumbing".to_string(),
            price_lamports: 500_000_000,
        }
        .data(),
    };

    let register_provider_transaction = Transaction::new_signed_with_payer(
        &[register_provider_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_provider_transaction).unwrap();

    let register_customer_accounts = project::accounts::RegisterCustomer {
        authority: customer_wallet.pubkey(),
        customer: customer_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_customer_instruction = Instruction {
        program_id: project::ID,
        accounts: register_customer_accounts.to_account_metas(None),
        data: project::instruction::RegisterCustomer {
            name: "Test Customer".to_string(),
        }
        .data(),
    };

    let register_customer_transaction = Transaction::new_signed_with_payer(
        &[register_customer_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_customer_transaction).unwrap();

    let booking_id = 1u64;

    let (booking_pda, _) = Pubkey::find_program_address(
        &[
            b"booking",
            customer_wallet.pubkey().as_ref(),
            booking_id.to_le_bytes().as_ref(),
        ],
        &project::ID,
    );

    let (escrow_pda, _) = Pubkey::find_program_address(
        &[b"escrow", booking_pda.as_ref()],
        &project::ID,
    );

    let create_booking_accounts = project::accounts::CreateBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
        escrow: escrow_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let create_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: create_booking_accounts.to_account_metas(None),
        data: project::instruction::CreateBooking {
            booking_id,
        }
        .data(),
    };

    let create_booking_transaction = Transaction::new_signed_with_payer(
        &[create_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_booking_transaction).unwrap();

    let booking_before = svm.get_account(&booking_pda).unwrap();

    let booking_before = Booking::try_deserialize(
        &mut booking_before.data.as_slice(),
    )
    .unwrap();

    assert_eq!(booking_before.status, BOOKING_CREATED);

    let accept_booking_accounts = project::accounts::AcceptBooking {
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
    };

    let accept_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: accept_booking_accounts.to_account_metas(None),
        data: project::instruction::AcceptBooking {
            booking_id,
        }
        .data(),
    };

    let accept_booking_transaction = Transaction::new_signed_with_payer(
        &[accept_booking_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(accept_booking_transaction).unwrap();

    let booking_account = svm.get_account(&booking_pda).unwrap();

    let booking = Booking::try_deserialize(
        &mut booking_account.data.as_slice(),
    )
    .unwrap();

    assert_eq!(booking.status, BOOKING_ACCEPTED);
    assert_eq!(booking.provider, provider_wallet.pubkey());
    assert_eq!(booking.customer, customer_wallet.pubkey());
}


#[test]
fn test_complete_job() {
    let mut svm = LiteSVM::new();

    let provider_wallet = Keypair::new();
    let customer_wallet = Keypair::new();

    svm.airdrop(&provider_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.airdrop(&customer_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (provider_pda, _) = Pubkey::find_program_address(
        &[b"provider", provider_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let (customer_pda, _) = Pubkey::find_program_address(
        &[b"customer", customer_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let register_provider_accounts = project::accounts::RegisterProvider {
        authority: provider_wallet.pubkey(),
        provider: provider_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_provider_instruction = Instruction {
        program_id: project::ID,
        accounts: register_provider_accounts.to_account_metas(None),
        data: project::instruction::RegisterProvider {
            name: "Test Provider".to_string(),
            service: "Plumbing".to_string(),
            price_lamports: 500_000_000,
        }
        .data(),
    };

    let register_provider_transaction = Transaction::new_signed_with_payer(
        &[register_provider_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_provider_transaction).unwrap();

    let register_customer_accounts = project::accounts::RegisterCustomer {
        authority: customer_wallet.pubkey(),
        customer: customer_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_customer_instruction = Instruction {
        program_id: project::ID,
        accounts: register_customer_accounts.to_account_metas(None),
        data: project::instruction::RegisterCustomer {
            name: "Test Customer".to_string(),
        }
        .data(),
    };

    let register_customer_transaction = Transaction::new_signed_with_payer(
        &[register_customer_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_customer_transaction).unwrap();

    let booking_id = 1u64;

    let (booking_pda, _) = Pubkey::find_program_address(
        &[
            b"booking",
            customer_wallet.pubkey().as_ref(),
            booking_id.to_le_bytes().as_ref(),
        ],
        &project::ID,
    );

    let (escrow_pda, _) = Pubkey::find_program_address(
        &[b"escrow", booking_pda.as_ref()],
        &project::ID,
    );

    let create_booking_accounts = project::accounts::CreateBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
        escrow: escrow_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let create_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: create_booking_accounts.to_account_metas(None),
        data: project::instruction::CreateBooking {
            booking_id,
        }
        .data(),
    };

    let create_booking_transaction = Transaction::new_signed_with_payer(
        &[create_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_booking_transaction).unwrap();

    let accept_booking_accounts = project::accounts::AcceptBooking {
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
    };

    let accept_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: accept_booking_accounts.to_account_metas(None),
        data: project::instruction::AcceptBooking {
            booking_id,
        }
        .data(),
    };

    let accept_booking_transaction = Transaction::new_signed_with_payer(
        &[accept_booking_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(accept_booking_transaction).unwrap();

    let booking_after_accept = svm.get_account(&booking_pda).unwrap();

    let booking_after_accept = Booking::try_deserialize(
        &mut booking_after_accept.data.as_slice(),
    )
    .unwrap();

    assert_eq!(booking_after_accept.status, BOOKING_ACCEPTED);

    let complete_job_accounts = project::accounts::CompleteJob {
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
    };

    let complete_job_instruction = Instruction {
        program_id: project::ID,
        accounts: complete_job_accounts.to_account_metas(None),
        data: project::instruction::CompleteJob {
            booking_id,
        }
        .data(),
    };

    let complete_job_transaction = Transaction::new_signed_with_payer(
        &[complete_job_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(complete_job_transaction).unwrap();

    let booking_account = svm.get_account(&booking_pda).unwrap();

    let booking = Booking::try_deserialize(
        &mut booking_account.data.as_slice(),
    )
    .unwrap();

    assert_eq!(booking.status, BOOKING_COMPLETED);
    assert_eq!(booking.provider, provider_wallet.pubkey());
    assert_eq!(booking.customer, customer_wallet.pubkey());
}


#[test]
fn test_approve_job_and_release_payment() {
    let mut svm = LiteSVM::new();

    let provider_wallet = Keypair::new();
    let customer_wallet = Keypair::new();

    svm.airdrop(&provider_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.airdrop(&customer_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (provider_pda, _) = Pubkey::find_program_address(
        &[b"provider", provider_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let (customer_pda, _) = Pubkey::find_program_address(
        &[b"customer", customer_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let register_provider_accounts = project::accounts::RegisterProvider {
        authority: provider_wallet.pubkey(),
        provider: provider_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_provider_instruction = Instruction {
        program_id: project::ID,
        accounts: register_provider_accounts.to_account_metas(None),
        data: project::instruction::RegisterProvider {
            name: "Test Provider".to_string(),
            service: "Plumbing".to_string(),
            price_lamports: 500_000_000,
        }
        .data(),
    };

    let register_provider_transaction = Transaction::new_signed_with_payer(
        &[register_provider_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_provider_transaction).unwrap();

    let register_customer_accounts = project::accounts::RegisterCustomer {
        authority: customer_wallet.pubkey(),
        customer: customer_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_customer_instruction = Instruction {
        program_id: project::ID,
        accounts: register_customer_accounts.to_account_metas(None),
        data: project::instruction::RegisterCustomer {
            name: "Test Customer".to_string(),
        }
        .data(),
    };

    let register_customer_transaction = Transaction::new_signed_with_payer(
        &[register_customer_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_customer_transaction).unwrap();

    let booking_id = 1u64;
    let expected_amount = 500_000_000u64;

    let (booking_pda, _) = Pubkey::find_program_address(
        &[
            b"booking",
            customer_wallet.pubkey().as_ref(),
            booking_id.to_le_bytes().as_ref(),
        ],
        &project::ID,
    );

    let (escrow_pda, _) = Pubkey::find_program_address(
        &[b"escrow", booking_pda.as_ref()],
        &project::ID,
    );

    let create_booking_accounts = project::accounts::CreateBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
        escrow: escrow_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let create_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: create_booking_accounts.to_account_metas(None),
        data: project::instruction::CreateBooking {
            booking_id,
        }
        .data(),
    };

    let create_booking_transaction = Transaction::new_signed_with_payer(
        &[create_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_booking_transaction).unwrap();

    let accept_booking_accounts = project::accounts::AcceptBooking {
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
    };

    let accept_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: accept_booking_accounts.to_account_metas(None),
        data: project::instruction::AcceptBooking {
            booking_id,
        }
        .data(),
    };

    let accept_booking_transaction = Transaction::new_signed_with_payer(
        &[accept_booking_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(accept_booking_transaction).unwrap();

    let complete_job_accounts = project::accounts::CompleteJob {
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
    };

    let complete_job_instruction = Instruction {
        program_id: project::ID,
        accounts: complete_job_accounts.to_account_metas(None),
        data: project::instruction::CompleteJob {
            booking_id,
        }
        .data(),
    };

    let complete_job_transaction = Transaction::new_signed_with_payer(
        &[complete_job_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(complete_job_transaction).unwrap();

    let booking_before_approval = svm.get_account(&booking_pda).unwrap();

    let booking_before_approval = Booking::try_deserialize(
        &mut booking_before_approval.data.as_slice(),
    )
    .unwrap();

    assert_eq!(
        booking_before_approval.status,
        BOOKING_COMPLETED
    );

    let provider_balance_before = svm
        .get_account(&provider_wallet.pubkey())
        .unwrap()
        .lamports;

    let approve_job_accounts = project::accounts::ApproveJob {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        provider_authority: provider_wallet.pubkey(),
        booking: booking_pda,
        escrow: escrow_pda,
    };

    let approve_job_instruction = Instruction {
        program_id: project::ID,
        accounts: approve_job_accounts.to_account_metas(None),
        data: project::instruction::ApproveJob {
            booking_id,
        }
        .data(),
    };

    let approve_job_transaction = Transaction::new_signed_with_payer(
        &[approve_job_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(approve_job_transaction).unwrap();

    let provider_balance_after = svm
        .get_account(&provider_wallet.pubkey())
        .unwrap()
        .lamports;

    let booking_account = svm.get_account(&booking_pda).unwrap();

    let booking = Booking::try_deserialize(
        &mut booking_account.data.as_slice(),
    )
    .unwrap();

    assert_eq!(booking.status, BOOKING_APPROVED);
    assert_eq!(booking.provider, provider_wallet.pubkey());
    assert_eq!(booking.customer, customer_wallet.pubkey());

    assert_eq!(
        provider_balance_after - provider_balance_before,
        expected_amount
    );

    assert!(svm.get_account(&escrow_pda).is_none());
}



#[test]
fn test_cancel_booking_and_refund_customer() {
    let mut svm = LiteSVM::new();

    let provider_wallet = Keypair::new();
    let customer_wallet = Keypair::new();

    svm.airdrop(&provider_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.airdrop(&customer_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (provider_pda, _) = Pubkey::find_program_address(
        &[b"provider", provider_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let (customer_pda, _) = Pubkey::find_program_address(
        &[b"customer", customer_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let register_provider_accounts = project::accounts::RegisterProvider {
        authority: provider_wallet.pubkey(),
        provider: provider_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_provider_instruction = Instruction {
        program_id: project::ID,
        accounts: register_provider_accounts.to_account_metas(None),
        data: project::instruction::RegisterProvider {
            name: "Test Provider".to_string(),
            service: "Plumbing".to_string(),
            price_lamports: 500_000_000,
        }
        .data(),
    };

    let register_provider_transaction = Transaction::new_signed_with_payer(
        &[register_provider_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_provider_transaction).unwrap();

    let register_customer_accounts = project::accounts::RegisterCustomer {
        authority: customer_wallet.pubkey(),
        customer: customer_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_customer_instruction = Instruction {
        program_id: project::ID,
        accounts: register_customer_accounts.to_account_metas(None),
        data: project::instruction::RegisterCustomer {
            name: "Test Customer".to_string(),
        }
        .data(),
    };

    let register_customer_transaction = Transaction::new_signed_with_payer(
        &[register_customer_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_customer_transaction).unwrap();

    let booking_id = 1u64;
    let expected_amount = 500_000_000u64;

    let (booking_pda, _) = Pubkey::find_program_address(
        &[
            b"booking",
            customer_wallet.pubkey().as_ref(),
            booking_id.to_le_bytes().as_ref(),
        ],
        &project::ID,
    );

    let (escrow_pda, _) = Pubkey::find_program_address(
        &[b"escrow", booking_pda.as_ref()],
        &project::ID,
    );

    let create_booking_accounts = project::accounts::CreateBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
        escrow: escrow_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let create_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: create_booking_accounts.to_account_metas(None),
        data: project::instruction::CreateBooking {
            booking_id,
        }
        .data(),
    };

    let create_booking_transaction = Transaction::new_signed_with_payer(
        &[create_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_booking_transaction).unwrap();

    let customer_balance_before_cancel = svm
        .get_account(&customer_wallet.pubkey())
        .unwrap()
        .lamports;

    let cancel_booking_accounts = project::accounts::CancelBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        booking: booking_pda,
        escrow: escrow_pda,
    };

    let cancel_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: cancel_booking_accounts.to_account_metas(None),
        data: project::instruction::CancelBooking {
            booking_id,
        }
        .data(),
    };

    let cancel_booking_transaction = Transaction::new_signed_with_payer(
        &[cancel_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(cancel_booking_transaction).unwrap();

    let customer_balance_after_cancel = svm
        .get_account(&customer_wallet.pubkey())
        .unwrap()
        .lamports;

    let booking_account = svm.get_account(&booking_pda).unwrap();

    let booking = Booking::try_deserialize(
        &mut booking_account.data.as_slice(),
    )
    .unwrap();

    assert_eq!(booking.status, BOOKING_CANCELLED);
    assert_eq!(booking.amount_lamports, expected_amount);

    assert!(
        customer_balance_after_cancel > customer_balance_before_cancel
    );

    assert!(svm.get_account(&escrow_pda).is_none());
}



#[test]
fn test_cannot_cancel_accepted_booking() {
    let mut svm = LiteSVM::new();

    let provider_wallet = Keypair::new();
    let customer_wallet = Keypair::new();

    svm.airdrop(&provider_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.airdrop(&customer_wallet.pubkey(), 10_000_000_000)
        .unwrap();

    svm.add_program(
        project::ID,
        include_bytes!("../../../target/deploy/project.so"),
    );

    let (provider_pda, _) = Pubkey::find_program_address(
        &[b"provider", provider_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let (customer_pda, _) = Pubkey::find_program_address(
        &[b"customer", customer_wallet.pubkey().as_ref()],
        &project::ID,
    );

    let register_provider_accounts = project::accounts::RegisterProvider {
        authority: provider_wallet.pubkey(),
        provider: provider_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_provider_instruction = Instruction {
        program_id: project::ID,
        accounts: register_provider_accounts.to_account_metas(None),
        data: project::instruction::RegisterProvider {
            name: "Test Provider".to_string(),
            service: "Plumbing".to_string(),
            price_lamports: 500_000_000,
        }
        .data(),
    };

    let register_provider_transaction = Transaction::new_signed_with_payer(
        &[register_provider_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_provider_transaction).unwrap();

    let register_customer_accounts = project::accounts::RegisterCustomer {
        authority: customer_wallet.pubkey(),
        customer: customer_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let register_customer_instruction = Instruction {
        program_id: project::ID,
        accounts: register_customer_accounts.to_account_metas(None),
        data: project::instruction::RegisterCustomer {
            name: "Test Customer".to_string(),
        }
        .data(),
    };

    let register_customer_transaction = Transaction::new_signed_with_payer(
        &[register_customer_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(register_customer_transaction).unwrap();

    let booking_id = 1u64;

    let (booking_pda, _) = Pubkey::find_program_address(
        &[
            b"booking",
            customer_wallet.pubkey().as_ref(),
            booking_id.to_le_bytes().as_ref(),
        ],
        &project::ID,
    );

    let (escrow_pda, _) = Pubkey::find_program_address(
        &[b"escrow", booking_pda.as_ref()],
        &project::ID,
    );

    let create_booking_accounts = project::accounts::CreateBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
        escrow: escrow_pda,
        system_program: anchor_lang::system_program::ID,
    };

    let create_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: create_booking_accounts.to_account_metas(None),
        data: project::instruction::CreateBooking {
            booking_id,
        }
        .data(),
    };

    let create_booking_transaction = Transaction::new_signed_with_payer(
        &[create_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(create_booking_transaction).unwrap();

    let accept_booking_accounts = project::accounts::AcceptBooking {
        provider_authority: provider_wallet.pubkey(),
        provider: provider_pda,
        booking: booking_pda,
    };

    let accept_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: accept_booking_accounts.to_account_metas(None),
        data: project::instruction::AcceptBooking {
            booking_id,
        }
        .data(),
    };

    let accept_booking_transaction = Transaction::new_signed_with_payer(
        &[accept_booking_instruction],
        Some(&provider_wallet.pubkey()),
        &[&provider_wallet],
        svm.latest_blockhash(),
    );

    svm.send_transaction(accept_booking_transaction).unwrap();

    let cancel_booking_accounts = project::accounts::CancelBooking {
        customer_authority: customer_wallet.pubkey(),
        customer: customer_pda,
        booking: booking_pda,
        escrow: escrow_pda,
    };

    let cancel_booking_instruction = Instruction {
        program_id: project::ID,
        accounts: cancel_booking_accounts.to_account_metas(None),
        data: project::instruction::CancelBooking {
            booking_id,
        }
        .data(),
    };

    let cancel_booking_transaction = Transaction::new_signed_with_payer(
        &[cancel_booking_instruction],
        Some(&customer_wallet.pubkey()),
        &[&customer_wallet],
        svm.latest_blockhash(),
    );

    assert!(
        svm.send_transaction(cancel_booking_transaction).is_err()
    );

    let booking_account = svm.get_account(&booking_pda).unwrap();

    let booking = Booking::try_deserialize(
        &mut booking_account.data.as_slice(),
    )
    .unwrap();

    assert_eq!(booking.status, BOOKING_ACCEPTED);
    assert!(svm.get_account(&escrow_pda).is_some());
}

