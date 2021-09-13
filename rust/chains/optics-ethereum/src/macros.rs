/// Dispatches a transaction, logs the tx id, and returns the result
#[macro_export]
macro_rules! report_tx {
    ($tx:expr) => {{

        // "0x..."
        let data = format!("0x{}", hex::encode(&$tx.tx.data().map(|b| b.to_vec()).unwrap_or_default()));

        tracing::info!(
            to = ?$tx.tx.to(),
            data = %data,
            nonce = ?$tx.tx.nonce(),
            "Dispatching transaction"
        );
        let dispatch_fut = $tx.send();
        let dispatched = dispatch_fut.await?;

        let tx_hash: ethers::core::types::H256 = *dispatched;

        tracing::info!(
            to = ?$tx.tx.to(),
            data = ?$tx.tx.data(),
            nonce = ?$tx.tx.nonce(),
            tx_hash = ?tx_hash,
            "Dispatched tx with tx_hash {:?}",
            tx_hash
        );

        let result = dispatched
            .await?
            .ok_or_else(|| optics_core::traits::ChainCommunicationError::DroppedError(tx_hash))?;

        tracing::info!(
            "confirmed transaction with tx_hash {:?}",
            result.transaction_hash
        );
        result
    }};
}
