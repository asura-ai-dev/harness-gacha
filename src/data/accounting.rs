use std::collections::HashMap;
use std::path::Path;

use crate::models::accounting::{AccountingData, Transaction};

pub fn load_accounting(path: &Path) -> AccountingData {
    match std::fs::read_to_string(path) {
        Ok(content) => serde_json::from_str::<AccountingData>(&content).unwrap_or_else(|e| {
            eprintln!("accounting.json パースエラー: {}", e);
            default_accounting()
        }),
        Err(_) => default_accounting(),
    }
}

pub fn save_accounting(path: &Path, data: &AccountingData) -> Result<(), String> {
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Accounting データのシリアライズに失敗しました: {}", e))?;

    std::fs::write(path, content)
        .map_err(|e| format!("Accounting データの書き込みに失敗しました: {}", e))
}

pub fn default_accounting() -> AccountingData {
    AccountingData {
        transactions: Vec::new(),
        creator_shares: HashMap::new(),
    }
}

pub fn gross_sales_for_pack(data: &AccountingData, pack_id: &str) -> i64 {
    data.transactions
        .iter()
        .filter(|tx| tx.pack_id == pack_id && tx.tx_type == "purchase")
        .map(|tx| tx.amount)
        .sum()
}

pub fn refunds_for_pack(data: &AccountingData, pack_id: &str) -> i64 {
    data.transactions
        .iter()
        .filter(|tx| tx.pack_id == pack_id && tx.tx_type == "refund")
        .map(|tx| tx.amount)
        .sum()
}

pub fn net_sales_for_pack(data: &AccountingData, pack_id: &str) -> i64 {
    gross_sales_for_pack(data, pack_id) - refunds_for_pack(data, pack_id)
}

pub fn transactions_for_month<'a>(
    data: &'a AccountingData,
    year_month: &str,
) -> Vec<&'a Transaction> {
    data.transactions
        .iter()
        .filter(|tx| tx.timestamp.starts_with(year_month))
        .collect()
}

pub fn calculate_monthly_payouts(data: &AccountingData, year_month: &str) -> HashMap<String, i64> {
    let month_transactions = transactions_for_month(data, year_month);
    let mut payouts = HashMap::new();

    for (creator_name, share) in &data.creator_shares {
        let creator_total = share
            .packs
            .iter()
            .map(|pack_id| {
                let gross: i64 = month_transactions
                    .iter()
                    .filter(|tx| tx.pack_id == *pack_id && tx.tx_type == "purchase")
                    .map(|tx| tx.amount)
                    .sum();
                let refunds: i64 = month_transactions
                    .iter()
                    .filter(|tx| tx.pack_id == *pack_id && tx.tx_type == "refund")
                    .map(|tx| tx.amount)
                    .sum();
                let net = gross - refunds;
                (net as f64 * share.share_rate) as i64
            })
            .sum();

        payouts.insert(creator_name.clone(), creator_total);
    }

    payouts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::accounting::CreatorShare;

    fn sample_data() -> AccountingData {
        let mut creator_shares = HashMap::new();
        creator_shares.insert(
            "Creator A".to_string(),
            CreatorShare {
                share_rate: 0.7,
                packs: vec!["pack-a".to_string()],
            },
        );
        creator_shares.insert(
            "Creator B".to_string(),
            CreatorShare {
                share_rate: 0.6,
                packs: vec!["pack-b".to_string()],
            },
        );

        AccountingData {
            transactions: vec![
                Transaction {
                    pack_id: "pack-a".to_string(),
                    user_id: "user-1".to_string(),
                    amount: 1000,
                    tx_type: "purchase".to_string(),
                    timestamp: "2026-04-05T10:00:00Z".to_string(),
                    stripe_payment_id: None,
                },
                Transaction {
                    pack_id: "pack-a".to_string(),
                    user_id: "user-2".to_string(),
                    amount: 1000,
                    tx_type: "purchase".to_string(),
                    timestamp: "2026-04-10T10:00:00Z".to_string(),
                    stripe_payment_id: None,
                },
                Transaction {
                    pack_id: "pack-a".to_string(),
                    user_id: "user-1".to_string(),
                    amount: 1000,
                    tx_type: "refund".to_string(),
                    timestamp: "2026-04-12T10:00:00Z".to_string(),
                    stripe_payment_id: None,
                },
                Transaction {
                    pack_id: "pack-b".to_string(),
                    user_id: "user-3".to_string(),
                    amount: 2000,
                    tx_type: "purchase".to_string(),
                    timestamp: "2026-04-15T10:00:00Z".to_string(),
                    stripe_payment_id: None,
                },
            ],
            creator_shares,
        }
    }

    #[test]
    fn test_gross_sales() {
        let data = sample_data();

        assert_eq!(gross_sales_for_pack(&data, "pack-a"), 2000);
        assert_eq!(gross_sales_for_pack(&data, "pack-b"), 2000);
        assert_eq!(gross_sales_for_pack(&data, "nonexistent"), 0);
    }

    #[test]
    fn test_refunds() {
        let data = sample_data();

        assert_eq!(refunds_for_pack(&data, "pack-a"), 1000);
        assert_eq!(refunds_for_pack(&data, "pack-b"), 0);
    }

    #[test]
    fn test_net_sales() {
        let data = sample_data();

        assert_eq!(net_sales_for_pack(&data, "pack-a"), 1000);
        assert_eq!(net_sales_for_pack(&data, "pack-b"), 2000);
    }

    #[test]
    fn test_monthly_payouts() {
        let data = sample_data();
        let payouts = calculate_monthly_payouts(&data, "2026-04");

        assert_eq!(payouts.get("Creator A"), Some(&700));
        assert_eq!(payouts.get("Creator B"), Some(&1200));
    }

    #[test]
    fn test_monthly_payouts_empty_month() {
        let data = sample_data();
        let payouts = calculate_monthly_payouts(&data, "2026-05");

        assert_eq!(payouts.get("Creator A"), Some(&0));
        assert_eq!(payouts.get("Creator B"), Some(&0));
    }
}
