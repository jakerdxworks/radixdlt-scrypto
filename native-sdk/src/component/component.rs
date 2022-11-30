use radix_engine_interface::api::api::{EngineApi, SysNativeInvokable};
use radix_engine_interface::api::types::{ComponentId, RENodeId};
use radix_engine_interface::data::ScryptoDecode;
use radix_engine_interface::model::*;
use sbor::rust::fmt::Debug;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Component(pub ComponentId);

impl Component {
    pub fn sys_add_access_check<Y, E: Debug + ScryptoDecode>(
        &mut self,
        access_rules: AccessRules,
        sys_calls: &mut Y,
    ) -> Result<&mut Self, E>
    where
        Y: EngineApi<E> + SysNativeInvokable<AccessRulesAddAccessCheckInvocation, E>,
    {
        sys_calls.sys_invoke(AccessRulesAddAccessCheckInvocation {
            receiver: RENodeId::Component(self.0),
            access_rules,
        })?;

        Ok(self)
    }

    pub fn sys_set_royalty_config<Y, E: Debug + ScryptoDecode>(
        &mut self,
        royalty_config: RoyaltyConfig,
        sys_calls: &mut Y,
    ) -> Result<&mut Self, E>
    where
        Y: EngineApi<E> + SysNativeInvokable<ComponentSetRoyaltyConfigInvocation, E>,
    {
        sys_calls.sys_invoke(ComponentSetRoyaltyConfigInvocation {
            receiver: RENodeId::Component(self.0),
            royalty_config,
        })?;

        Ok(self)
    }
}
