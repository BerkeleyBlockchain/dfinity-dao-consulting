import React, { useRef, useState } from "react";
import { Dialog } from "@headlessui/react";

import { TOKENS_CANISTER, useIdentity, WICP_CANISTER } from "./Identity";
import StakeIcon from "./svg/stake-solid.svg";
import BigCross from "./svg/big-cross.svg";
import Loader from "./svg/puff-loader.svg";

const NS_PER_HOUR = 3600000000000n;

const Stake = () => {
  const { identity } = useIdentity();
  const [showStakeModal, setShowStakeModal] = useState<boolean>(false);
  const [stakeAmount, setStakeAmount] = useState<string>("");
  const [isLoading, setIsLoading] = useState<boolean>(false);
  const closeButtonRef = useRef(null);

  const stake = async (amount: bigint) => {
    setIsLoading(true);
    const result = await identity.wipc.approve(TOKENS_CANISTER, amount);
    // TODO: error handling
    await identity.tokens.joinAsVoter(amount, 24n * 7n * NS_PER_HOUR);
    setIsLoading(false);
  };

  return (
    <>
      <button
        className="transition transform-gpu transform-none inline-flex items-center
        hover:scale-105 active:scale-95"
        onClick={() => setShowStakeModal(true)}
      >
        <span className="text-stone-100 text-lg font-medium">{`10 GGT`} </span>
        <span
          className="transition ml-4 inline-flex items-center justify-center
        text-emerald-400 hover:text-emerald-300
        "
        >
          <StakeIcon className="h-4 w-4 mr-1" />
          <span className="text-sm">Stake</span>
        </span>
      </button>

      <Dialog
        initialFocus={closeButtonRef}
        open={showStakeModal}
        onClose={() => setShowStakeModal(false)}
        className="fixed z-10 inset-0 overflow-y-auto"
      >
        <div className="flex items-center justify-center min-h-screen px-4">
          <Dialog.Overlay className="fixed inset-0 overflow-y-auto backdrop-blur-lg" />
          <div className="relative bg-stone-900 max-w-md shadow-xl rounded-2xl p-6">
            <div className="flex items-center">
              <Dialog.Title className="flex-1 text-stone-100 text-xl leading-6 font-medium">
                Stake ICP
              </Dialog.Title>
              <button
                ref={closeButtonRef}
                className="transition ml-8 p-1 rounded
                hover:bg-stone-700 active:scale-95"
                onClick={() => setShowStakeModal(false)}
              >
                <BigCross className="h-5 w-5" />
              </button>
            </div>

            <Dialog.Description className="mt-2 text-sm">
              Stake WICP to recieve governance tokens to vote on proposals.
              Depending on the length of time you stake your ICP for, you will
              recieve staking rewards.
            </Dialog.Description>

            <div className="mt-4">
              <label
                htmlFor="stake-amount"
                className="block text-sm font-medium text-stone-300"
              >
                Stake Amount
              </label>
              <div className="mt-2 flex items-stretch">
                <div className="flex-1 relative">
                  {/* <div
                    className="absolute inset-y-0 left-0 pl-3 mt-[-2px] flex items-center pointer-events-none
                  text-stone-500 font-mono text-xl"
                  >
                    $
                  </div> */}
                  <input
                    type="text"
                    inputMode="numeric"
                    pattern="[0-9]*"
                    name="stake-amount"
                    className="focus:outline-none focus:ring focus:ring-amber-600 focus:border-amber-600
                    block w-full pl-4 pr-12 py-2 shadow-md
                    appearance-none hover:appearance-none
                    text-2xl text-stone-100 bg-stone-800 font-mono rounded-md"
                    placeholder="0.00"
                    value={stakeAmount}
                    disabled={isLoading}
                    onChange={(e) =>
                      setStakeAmount(e.target.value.replace(/\D/g, ""))
                    }
                  />
                  <div className="absolute inset-y-0 right-0 pr-3 flex items-center">
                    WICP
                  </div>
                </div>
                <button
                  className="transition ml-2 rounded-md flex items-center justify-center
                  px-4 py-2 w-24
                  bg-emerald-500 hover:bg-emerald-400 text-white shadow
                  hover:scale-105 active:scale-95 transform-gpu
                  disabled:hover:scale-100 disabled:active:scale-100
                  disabled:hover:bg-stone-500 disabled:bg-stone-500"
                  onClick={() => stake(BigInt(stakeAmount))}
                  disabled={isLoading}
                >
                  {isLoading ? (
                    <Loader className="h-5 w-5" />
                  ) : (
                    <>
                      <StakeIcon className="h-4 w-4 mr-1" />
                      Stake
                    </>
                  )}
                </button>
              </div>
            </div>
          </div>
        </div>
      </Dialog>
    </>
  );
};

export default Stake;
