# meco
# Built with Seahorse v0.2.0

from seahorse.prelude import *

declare_id('Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS')

''' The prediction market is a simple two-function market that operates on-chain. Users will be able to:
        1. Create a prediction market auction associated with their wallet
        2. Perform two main operations - bid and withdraw/claim - on the prediction set in the market auction
        3. Close/end the auction after a certain time period.
'''

class Market(Account):
    owner: Pubkey
    display: i64

class Operation(Enum):
    BID = 0
    CLAIM = 1


class Calculator(Account):
    owner: Pubkey
    display: i64

# @instruction
# def initialiseMarket(owner: Signer, market: Empty[Market]):
#     market = market.init(payer=owner, seeds=['Market', owner])
#     market.owner = owner.key()

@instruction
def init_calculator(owner: Signer, calculator: Empty[Calculator]):
    calculator = calculator.init(payer = owner, seeds = ['Calculator', owner])
    calculator.owner = owner.key()