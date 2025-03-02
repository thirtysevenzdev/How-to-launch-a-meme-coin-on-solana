use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};
use solana_client::rpc_client::RpcClient;
use spl_token::instruction::initialize_mint;
use spl_token::state::Mint;
use solana_program::program_pack::Pack; // Правильный импорт для Pack
use std::io;
use std::collections::HashMap;

// Определение структуры Token
#[derive(Clone)]
struct Token {
    name: String,
    symbol: String,
    mint_address: Pubkey,
    description: Option<String>,
    logo: Option<String>,
    website: Option<String>,
    telegram: Option<String>,
    twitter: Option<String>,
    discord: Option<String>,
    creator_name: Option<String>,
    creator_website: Option<String>,
    freeze_authority: bool,
    mint_authority: bool,
}

// Функция для создания нового кошелька
fn create_new_wallet() -> Keypair {
    let keypair = Keypair::new();
    println!("Создан новый кошелек!");
    println!("Публичный ключ: {}", keypair.pubkey());
    println!("Приватный ключ (Base58): {}", keypair.to_base58_string());
    keypair
}

// Функция для расчета и отображения необходимого количества SOL для создания токена
fn display_required_sol_for_token_creation(rpc_client: &RpcClient) -> u64 {
    let rent_exemption = rpc_client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap_or(0);
    let fee_lamports = 5000; // Примерное значение комиссии
    let total_required = rent_exemption + fee_lamports;
    println!("Необходимо пополнить новый кошелек как минимум на {} SOL для создания токена.", total_required as f64 / 1_000_000_000.0);
    total_required
}

// Функция для проверки баланса кошелька
fn check_wallet_balance(rpc_client: &RpcClient, pubkey: &Pubkey) -> u64 {
    match rpc_client.get_balance(pubkey) {
        Ok(balance) => balance,
        Err(e) => {
            println!("Не удалось получить баланс кошелька: {}", e);
            0
        }
    }
}

// Функция для создания токена
fn create_token(rpc_client: &RpcClient, mint_keypair: &Keypair, _tokens: &mut HashMap<String, Token>) {
    // Логика создания токена
    let mint_pubkey = mint_keypair.pubkey();
    println!("Создание токена с адресом: {}", mint_pubkey);

    // Создаем инструкцию для инициализации токена
    let instruction = initialize_mint(
        &spl_token::id(),
        &mint_pubkey,
        &mint_keypair.pubkey(),
        Some(&mint_keypair.pubkey()),
        9, // Количество десятичных знаков
    ).unwrap();

    // Создаем и отправляем транзакцию
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&mint_keypair.pubkey()),
        &[mint_keypair],
        rpc_client.get_latest_blockhash().unwrap(),
    );

    match rpc_client.send_and_confirm_transaction(&transaction) {
        Ok(_) => println!("Токен успешно создан!"),
        Err(e) => println!("Не удалось создать токен: {}", e),
    }
}

// Основная функция
#[tokio::main]
async fn main() {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());

    println!("Добро пожаловать в Solana Token Creator!");

    let mut tokens = HashMap::new();

    loop {
        println!("Выберите опцию:");
        println!("1. Создать новый токен");
        println!("2. Выйти");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Не удалось прочитать строку");

        match choice.trim() {
            "1" => {
                // Создаем новый Keypair для токена
                let mint_keypair = create_new_wallet();
                
                // Отображаем необходимое количество SOL для создания токена
                let required_sol = display_required_sol_for_token_creation(&rpc_client);
                
                // Просим пользователя пополнить кошелек
                println!("Пополните кошелек и нажмите Enter для продолжения...");
                let mut proceed = String::new();
                io::stdin().read_line(&mut proceed).expect("Не удалось прочитать строку");

                // Проверяем, пополнен ли кошелек
                let balance = check_wallet_balance(&rpc_client, &mint_keypair.pubkey());
                if balance >= required_sol {
                    // Создаем токен
                    create_token(&rpc_client, &mint_keypair, &mut tokens);
                } else {
                    println!("Кошелек недостаточно пополнен. Пожалуйста, пополните кошелек и попробуйте снова.");
                }
            }
            "2" => break,
            _ => println!("Неверная опция."),
        }
    }
}
