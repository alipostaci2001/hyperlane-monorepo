// SPDX-License-Identifier: MIT OR Apache-2.0
pragma solidity >=0.6.11;

library QueueLib {
    struct Queue {
        uint256 first;
        uint256 last;
        mapping(uint256 => bytes32) queue;
    }

    function init(Queue storage _q) internal {
        if (_q.first == 0) {
            _q.first = 1;
        }
    }

    function contains(Queue storage _q, bytes32 _item)
        internal
        view
        returns (bool)
    {
        for (uint256 i = _q.first; i < _q.last; i++) {
            if (_q.queue[i] == _item) {
                return true;
            }
        }
        return false;
    }

    function enqueue(Queue storage _q, bytes32 _item) internal {
        uint256 _last = _q.last + 1;
        _q.last = _last;
        _q.queue[_last] = _item;
    }

    function dequeue(Queue storage _q) internal returns (bytes32 item) {
        uint256 _first = _q.first;
        require(_q.last >= _first, "Empty");
        item = _q.queue[_first];
        delete _q.queue[_first];
        _q.first = _first + 1;
    }

    function length(Queue storage _q) internal view returns (uint256) {
        // Cannot underflow unless state is corrupted
        return _q.first - _q.last - 1;
    }
}

contract QueueManager {
    using QueueLib for QueueLib.Queue;
    QueueLib.Queue internal queue;

    constructor() {
        queue.init();
    }
}