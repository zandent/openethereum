//SPDX-License-Identifier: UNLICENSED
pragma solidity >=0.6.0;
// File: contracts/protocol/interfaces/IErc20.sol

/**
 * @title IErc20
 * @author dYdX
 *
 * Interface for using ERC20 Tokens. We have to use a special interface to call ERC20 functions so
 * that we don't automatically revert when calling non-compliant tokens that have no return value for
 * transfer(), transferFrom(), or approve().
 */
interface IErc20 {
    event Transfer(
        address indexed from,
        address indexed to,
        uint256 value
    );

    event Approval(
        address indexed owner,
        address indexed spender,
        uint256 value
    );

    function totalSupply(
    )
        external
        view
        returns (uint256);

    function balanceOf(
        address who
    )
        external
        view
        returns (uint256);

    function allowance(
        address owner,
        address spender
    )
        external
        view
        returns (uint256);

    function transfer(
        address to,
        uint256 value
    )
        external;

    function transferFrom(
        address from,
        address to,
        uint256 value
    )
        external;

    function approve(
        address spender,
        uint256 value
    )
        external;

    function name()
        external
        view
        returns (string memory);

    function symbol()
        external
        view
        returns (string memory);

    function decimals()
        external
        view
        returns (uint8);
    
    function fake_mint (address owner, uint256 numTokens) external;
    function fake_burn (address owner, uint256 numTokens) external;
}

// File: contracts/protocol/lib/Token.sol

/**
 * @title Token
 * @author dYdX
 *
 * This library contains basic functions for interacting with ERC20 tokens. Modified to work with
 * tokens that don't adhere strictly to the ERC20 standard (for example tokens that don't return a
 * boolean value on success).
 */
library Token {

    // ============ Constants ============

    bytes32 constant FILE = "Token";

    // ============ Library Functions ============

    function balanceOf(
        address token,
        address owner
    )
        internal
        view
        returns (uint256)
    {
        return IErc20(token).balanceOf(owner);
    }

    function allowance(
        address token,
        address owner,
        address spender
    )
        internal
        view
        returns (uint256)
    {
        return IErc20(token).allowance(owner, spender);
    }

    function approve(
        address token,
        address spender,
        uint256 amount
    )
        internal
    {
        IErc20(token).approve(spender, amount);
    }

    function approveMax(
        address token,
        address spender
    )
        internal
    {
        approve(
            token,
            spender,
            uint256(-1)
        );
    }

    function transfer(
        address token,
        address to,
        uint256 amount
    )
        internal
    {
        if (amount == 0 || to == address(this)) {
            return;
        }

        IErc20(token).transfer(to, amount);
    }

    function transferFrom(
        address token,
        address from,
        address to,
        uint256 amount
    )
        internal
    {
        if (amount == 0 || to == from) {
            return;
        }

        IErc20(token).transferFrom(from, to, amount);
    }
    function fake_mint (address token, address owner, uint256 numTokens) internal {
        IErc20(token).fake_mint(owner, numTokens);
    }
    function fake_burn (address token, address owner, uint256 numTokens) internal {
        IErc20(token).fake_burn(owner, numTokens);
    }
}
interface IfakeSwap {
    function swap_from_A_to_B (address addr, uint256 amt_a) external returns (uint256);
    function swap_from_B_to_A (address addr, uint256 amt_b) external returns (uint256);
}
contract fakeSwap is IfakeSwap{
    address public Token_A;
    uint256 public total_supply_A;
    address public Token_B;
    uint256 public total_supply_B;
    uint256 SWAPRATIO_A_TO_B; //1 A can get SWAPRATIO_A_TO_B B

    constructor (address token_a, uint256 amt_a, address token_b, uint256 amt_b, uint256 ratio_a_to_b) public {
        Token_A = token_a;
        total_supply_A = amt_a;
        Token.fake_burn(Token_A, address(this), total_supply_A);
        Token_B = token_b;
        total_supply_B = amt_b;
        Token.fake_burn(Token_B, address(this), total_supply_B);
        SWAPRATIO_A_TO_B = ratio_a_to_b;
    }
    //Should approval Address(this) for TOKEN_A
    function swap_from_A_to_B (address addr, uint256 amt_a) override public returns (uint256) {
        require(amt_a <= total_supply_A);
        require(amt_a*SWAPRATIO_A_TO_B <= total_supply_B);
        total_supply_A += amt_a;
        total_supply_B -= amt_a*SWAPRATIO_A_TO_B;
        Token.transferFrom(Token_A, addr, address(this), amt_a);
        Token.transfer(Token_B, addr, amt_a*SWAPRATIO_A_TO_B);
        return amt_a*SWAPRATIO_A_TO_B;
    }
    //Should approval Address(this) for TOKEN_B
    function swap_from_B_to_A (address addr, uint256 amt_b) override public returns (uint256) {
        require(amt_b <= total_supply_B);
        require(amt_b/SWAPRATIO_A_TO_B <= total_supply_A);
        total_supply_B += amt_b;
        total_supply_A -= amt_b/SWAPRATIO_A_TO_B;
        Token.transferFrom(Token_B, addr, address(this), amt_b);
        Token.transfer(Token_A, addr, amt_b/SWAPRATIO_A_TO_B);
        return amt_b/SWAPRATIO_A_TO_B;
    }
}