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

contract fakeSoloMargin {
    address public Token_A;
    uint256 public total_supply;
    constructor (address token_a, uint256 amt) public {
        Token_A = token_a;
        total_supply = amt;
        Token.fake_burn(Token_A, address(this), total_supply);
    }
    function operate (address addr, bytes memory data, uint256 loan, address token_a) public {
        require(loan <= total_supply);
        require(Token_A == token_a);
        Token.transfer(Token_A, addr, loan);
        (bool success, bytes memory returnData) = address(addr).call(data);
        require(success);
        Token.transferFrom(Token_A, addr, address(this), loan);
    }
}