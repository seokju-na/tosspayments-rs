use convert_case::{Case, Casing};
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum ErrorCode {
  AlreadyProcessedPayment,
  ProviderError,
  ExceedMaxCardInstallmentPlan,
  InvalidRequest,
  NotAllowedPointUse,
  InvalidApiKey,
  InvalidRejectCard,
  BelowMinimumAmount,
  InvalidCardExpiration,
  InvalidStoppedCard,
  ExceedMaxDailyPaymentCount,
  NotSupportedInstallmentPlanCardOrMerchant,
  InvalidCardInstallmentPlan,
  NotSupportedMonthlyInstallmentPlan,
  ExceedMaxPaymentAmount,
  NotFoundTerminalId,
  InvalidAuthorizeAuth,
  InvalidCardLostOrStolen,
  RestrictedTransferAccount,
  InvalidCardNumber,
  InvalidRegisteredSubmall,
  NotRegisteredBusiness,
  ExceedMaxOneDayWithdrawAmount,
  ExceedMaxOneTimeWithdrawAmount,
  CardProcessingError,
  ExceedMaxAmount,
  InvalidAccountInfoReRegister,
  NotAvailablePayment,
  UnapprovedOrderId,
  UnauthorizedKey,
  RejectAccountPayment,
  RejectCardPayment,
  RejectCardCompany,
  ForbiddenRequest,
  RejectTosspayInvalidAccount,
  ExceedMaxAuthCount,
  ExceedMaxOneDayAmount,
  NotAvailableBank,
  InvalidPassword,
  IncorrectBasicAuthFormat,
  FdsError,
  NotFound,
  NotFoundBilling,
  NotFoundPayment,
  NotFoundPaymentSession,
  FailedPaymentInternalSystemProcessing,
  FailedInternalSystemProcessing,
  UnknownPaymentError,
  AlreadyCanceledPayment,
  InvalidRefundAccountInfo,
  ExceedCancelAmountDiscountAmount,
  InvalidRefundAccountNumber,
  InvalidBank,
  NotMatchesRefundableAmount,
  RefundRejected,
  AlreadyRefundPayment,
  NotCancelableAmount,
  ForbiddenConsecutiveRequest,
  NotCancelablePayment,
  ExceedMaxRefundDue,
  NotAllowedPartialRefundWaitingDeposit,
  NotAllowedPartialRefund,
  NotCancelablePaymentForDormantUser,
  FailedRefundProcess,
  FailedMethodHandlingCancel,
  FailedPartialRefund,
  CommonError,
  NotSupportedCardType,
  InvalidCardPassword,
  InvalidCardIdentity,
  InvalidBirthDayFormat,
  NotRegisteredCardCompany,
  InvalidEmail,
  NotSupportedMethod,
  InvalidBillKeyRequest,
  DuplicatedOrderId,
  NotMatchesCustomerKey,
  FailedDbProcessing,
  FailedCardCompanyResponse,
  IdempotentRequestProcessing,
}

impl std::fmt::Display for ErrorCode {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", &format!("{:?}", self).to_case(Case::UpperSnake))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn error_code_display() {
    assert_eq!(format!("{}", ErrorCode::CommonError), "COMMON_ERROR");
    assert_eq!(
      format!("{}", ErrorCode::NotSupportedMethod),
      "NOT_SUPPORTED_METHOD"
    );
  }
}
