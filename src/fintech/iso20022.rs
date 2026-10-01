//! Modul Parser & Generator ISO 20022 XML untuk BI-FAST & National Payment Gateway
//! Mendukung pesan pacs.008 (Credit Transfer), pacs.002 (Payment Status Report), dan camt.053 (Bank Statement).

use crate::compiler::decimal::Desimal;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso20022Pacs008 {
    pub msg_id: String,
    pub created_at: String,
    pub instr_id: String,
    pub tx_id: String,
    pub amount: String,
    pub currency: String,
    pub debtor_name: String,
    pub debtor_account: String,
    pub debtor_bank_bic: String,
    pub creditor_name: String,
    pub creditor_account: String,
    pub creditor_bank_bic: String,
    pub settlement_method: String,
}

impl Iso20022Pacs008 {
    pub fn new(
        msg_id: &str,
        instr_id: &str,
        amount: Desimal,
        debtor_name: &str,
        debtor_acc: &str,
        creditor_name: &str,
        creditor_acc: &str,
    ) -> Self {
        Self {
            msg_id: msg_id.to_string(),
            created_at: "2026-10-01T08:00:00Z".to_string(),
            instr_id: instr_id.to_string(),
            tx_id: format!("TX-{}", msg_id),
            amount: amount.to_string(),
            currency: "IDR".to_string(),
            debtor_name: debtor_name.to_string(),
            debtor_account: debtor_acc.to_string(),
            debtor_bank_bic: "WIDYIDJA".to_string(),
            creditor_name: creditor_name.to_string(),
            creditor_account: creditor_acc.to_string(),
            creditor_bank_bic: "BINAIDJA".to_string(),
            settlement_method: "CLRG".to_string(),
        }
    }

    /// Serialisasi pesan ke format XML ISO 20022
    pub fn to_xml(&self) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.008.001.08">
  <FIToFICstmrCdtTrf>
    <GrpHdr>
      <MsgId>{}</MsgId>
      <CreDtTm>{}</CreDtTm>
      <NbOfTxs>1</NbOfTxs>
      <SttlmInf>
        <SttlmMtd>{}</SttlmMtd>
      </SttlmInf>
    </GrpHdr>
    <CdtTrfTxInf>
      <PmtId>
        <InstrId>{}</InstrId>
        <TxId>{}</TxId>
      </PmtId>
      <IntrBkSttlmAmt Ccy="{}">{}</IntrBkSttlmAmt>
      <Dbtr>
        <Nm>{}</Nm>
      </Dbtr>
      <DbtrAcct>
        <Id><Othr><Id>{}</Id></Othr></Id>
      </DbtrAcct>
      <DbtrAgt>
        <FinInstnId><BICFI>{}</BICFI></FinInstnId>
      </DbtrAgt>
      <CdtrAgt>
        <FinInstnId><BICFI>{}</BICFI></FinInstnId>
      </CdtrAgt>
      <Cdtr>
        <Nm>{}</Nm>
      </Cdtr>
      <CdtrAcct>
        <Id><Othr><Id>{}</Id></Othr></Id>
      </CdtrAcct>
    </CdtTrfTxInf>
  </FIToFICstmrCdtTrf>
</Document>"#,
            self.msg_id,
            self.created_at,
            self.settlement_method,
            self.instr_id,
            self.tx_id,
            self.currency,
            self.amount,
            self.debtor_name,
            self.debtor_account,
            self.debtor_bank_bic,
            self.creditor_bank_bic,
            self.creditor_name,
            self.creditor_account
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Iso20022Pacs002 {
    pub msg_id: String,
    pub original_msg_id: String,
    pub tx_status: String, // ACTC (Accepted), RJCT (Rejected), PDNG (Pending)
    pub reason_code: Option<String>,
}

impl Iso20022Pacs002 {
    pub fn new_accepted(msg_id: &str, orig_msg_id: &str) -> Self {
        Self {
            msg_id: msg_id.to_string(),
            original_msg_id: orig_msg_id.to_string(),
            tx_status: "ACTC".to_string(),
            reason_code: None,
        }
    }

    pub fn new_rejected(msg_id: &str, orig_msg_id: &str, reason: &str) -> Self {
        Self {
            msg_id: msg_id.to_string(),
            original_msg_id: orig_msg_id.to_string(),
            tx_status: "RJCT".to_string(),
            reason_code: Some(reason.to_string()),
        }
    }

    pub fn to_xml(&self) -> String {
        let reason_tag = if let Some(r) = &self.reason_code {
            format!("<StsRsnInf><Rsn><Cd>{}</Cd></Rsn></StsRsnInf>", r)
        } else {
            "".to_string()
        };

        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<Document xmlns="urn:iso:std:iso:20022:tech:xsd:pacs.002.001.10">
  <FIToFIPmtStsRpt>
    <GrpHdr>
      <MsgId>{}</MsgId>
      <CreDtTm>2026-10-01T08:00:01Z</CreDtTm>
    </GrpHdr>
    <TxInfAndSts>
      <OrgnlMsgId>{}</OrgnlMsgId>
      <TxSts>{}</TxSts>
      {}
    </TxInfAndSts>
  </FIToFIPmtStsRpt>
</Document>"#,
            self.msg_id, self.original_msg_id, self.tx_status, reason_tag
        )
    }
}
