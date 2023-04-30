use crate::blueprints::resource::{LocalRef, ProofError, ProofMoveableSubstate};
use crate::errors::RuntimeError;
use crate::types::*;
use radix_engine_interface::api::substate_lock_api::LockFlags;
use radix_engine_interface::api::ClientApi;
use radix_engine_interface::blueprints::resource::*;

#[derive(Debug, Clone, ScryptoSbor)]
pub struct FungibleProof {
    pub total_locked: Decimal,
    /// The supporting containers.
    pub evidence: BTreeMap<LocalRef, Decimal>,
}

impl FungibleProof {
    pub fn new(
        total_locked: Decimal,
        evidence: BTreeMap<LocalRef, Decimal>,
    ) -> Result<FungibleProof, ProofError> {
        if total_locked.is_zero() {
            return Err(ProofError::EmptyProofNotAllowed);
        }

        Ok(Self {
            total_locked,
            evidence,
        })
    }

    pub fn clone_proof<Y: ClientApi<RuntimeError>>(
        &self,
        api: &mut Y,
    ) -> Result<Self, RuntimeError> {
        for (container, locked_amount) in &self.evidence {
            api.call_method(
                container.as_node_id(),
                match container {
                    LocalRef::Bucket(_) => FUNGIBLE_BUCKET_LOCK_AMOUNT_IDENT,
                    LocalRef::Vault(_) => FUNGIBLE_VAULT_LOCK_FUNGIBLE_AMOUNT_IDENT,
                },
                scrypto_args!(locked_amount),
            )?;
        }
        Ok(Self {
            total_locked: self.total_locked.clone(),
            evidence: self.evidence.clone(),
        })
    }

    pub fn drop_proof<Y: ClientApi<RuntimeError>>(self, api: &mut Y) -> Result<(), RuntimeError> {
        for (container, locked_amount) in &self.evidence {
            api.call_method(
                container.as_node_id(),
                match container {
                    LocalRef::Bucket(_) => FUNGIBLE_BUCKET_UNLOCK_AMOUNT_IDENT,
                    LocalRef::Vault(_) => FUNGIBLE_VAULT_UNLOCK_FUNGIBLE_AMOUNT_IDENT,
                },
                scrypto_args!(locked_amount),
            )?;
        }
        Ok(())
    }

    pub fn amount(&self) -> Decimal {
        self.total_locked
    }
}

pub struct FungibleProofBlueprint;

impl FungibleProofBlueprint {
    pub(crate) fn clone<Y>(api: &mut Y) -> Result<Proof, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let moveable = {
            let handle =
                api.lock_field(FungibleProofOffset::Moveable.into(), LockFlags::read_only())?;
            let substate_ref: ProofMoveableSubstate = api.field_lock_read_typed(handle)?;
            let moveable = substate_ref.clone();
            api.field_lock_release(handle)?;
            moveable
        };

        let handle = api.lock_field(
            FungibleProofOffset::ProofRefs.into(),
            LockFlags::read_only(),
        )?;
        let substate_ref: FungibleProof = api.field_lock_read_typed(handle)?;
        let proof = substate_ref.clone();
        let clone = proof.clone_proof(api)?;

        let proof_id = api.new_object(
            FUNGIBLE_PROOF_BLUEPRINT,
            vec![
                scrypto_encode(&moveable).unwrap(),
                scrypto_encode(&clone).unwrap(),
            ],
        )?;

        // Drop after object creation to keep the reference alive
        api.field_lock_release(handle)?;

        Ok(Proof(Own(proof_id)))
    }

    pub(crate) fn get_amount<Y>(api: &mut Y) -> Result<Decimal, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let handle = api.lock_field(
            FungibleProofOffset::ProofRefs.into(),
            LockFlags::read_only(),
        )?;
        let substate_ref: FungibleProof = api.field_lock_read_typed(handle)?;
        let amount = substate_ref.amount();
        api.field_lock_release(handle)?;
        Ok(amount)
    }

    // TODO: Remove in favor of an API get_parent()
    pub(crate) fn get_resource_address<Y>(api: &mut Y) -> Result<ResourceAddress, RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        let address = ResourceAddress::new_or_panic(api.get_info()?.outer_object.unwrap().into());
        Ok(address)
    }

    pub(crate) fn drop<Y>(proof: Proof, api: &mut Y) -> Result<(), RuntimeError>
    where
        Y: ClientApi<RuntimeError>,
    {
        // FIXME: check type before schema check is ready! applicable to all functions!

        let parent = api
            .get_object_info(proof.0.as_node_id())?
            .outer_object
            .unwrap();

        api.call_method(
            parent.as_node_id(),
            RESOURCE_MANAGER_DROP_PROOF_IDENT,
            scrypto_encode(&ResourceManagerDropProofInput { proof }).unwrap(),
        )?;

        Ok(())
    }
}