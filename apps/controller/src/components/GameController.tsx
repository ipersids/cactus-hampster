import { useState, useEffect, useCallback, useRef } from 'react';
import type { ControllerInputPayload } from '@cactus-hampster/typeshare';

interface GameControllerProps {
  onInput: (input: ControllerInputPayload) => void;
}

type ButtonKey = 'thrust' | 'left' | 'right' | 'fire';

function GameController({ onInput }: GameControllerProps) {
  const [activeButtons, setActiveButtons] = useState<Set<ButtonKey>>(new Set());
  const lastInputRef = useRef<string>('');

  const handleTouchStart = useCallback((button: ButtonKey) => (e: React.TouchEvent | React.MouseEvent) => {
    e.preventDefault();
    setActiveButtons(prev => new Set(prev).add(button));
  }, []);

  const handleTouchEnd = useCallback((button: ButtonKey) => (e: React.TouchEvent | React.MouseEvent) => {
    e.preventDefault();
    setActiveButtons(prev => {
      const next = new Set(prev);
      next.delete(button);
      return next;
    });
  }, []);

  // Send input state whenever it changes
  useEffect(() => {
    const input: ControllerInputPayload = {
      thrust: activeButtons.has('thrust'),
      rotate_left: activeButtons.has('left'),
      rotate_right: activeButtons.has('right'),
      fire: activeButtons.has('fire'),
    };

    const inputKey = JSON.stringify(input);
    if (inputKey !== lastInputRef.current) {
      lastInputRef.current = inputKey;
      onInput(input);
    }
  }, [activeButtons, onInput]);

  // Also send periodic updates while buttons are held
  useEffect(() => {
    if (activeButtons.size === 0) return;

    const interval = setInterval(() => {
      const input: ControllerInputPayload = {
        thrust: activeButtons.has('thrust'),
        rotate_left: activeButtons.has('left'),
        rotate_right: activeButtons.has('right'),
        fire: activeButtons.has('fire'),
      };
      onInput(input);
    }, 50); // 20 updates per second while holding

    return () => clearInterval(interval);
  }, [activeButtons, onInput]);

  return (
    <div className="game-controller">
      <div className="controls-left">
        <button
          className={`control-btn rotate-btn ${activeButtons.has('left') ? 'active' : ''}`}
          onTouchStart={handleTouchStart('left')}
          onTouchEnd={handleTouchEnd('left')}
          onTouchCancel={handleTouchEnd('left')}
          onMouseDown={handleTouchStart('left')}
          onMouseUp={handleTouchEnd('left')}
          onMouseLeave={handleTouchEnd('left')}
        >
          &#x21BA;
        </button>
        <button
          className={`control-btn rotate-btn ${activeButtons.has('right') ? 'active' : ''}`}
          onTouchStart={handleTouchStart('right')}
          onTouchEnd={handleTouchEnd('right')}
          onTouchCancel={handleTouchEnd('right')}
          onMouseDown={handleTouchStart('right')}
          onMouseUp={handleTouchEnd('right')}
          onMouseLeave={handleTouchEnd('right')}
        >
          &#x21BB;
        </button>
      </div>

      <div className="controls-right">
        <button
          className={`control-btn thrust-btn ${activeButtons.has('thrust') ? 'active' : ''}`}
          onTouchStart={handleTouchStart('thrust')}
          onTouchEnd={handleTouchEnd('thrust')}
          onTouchCancel={handleTouchEnd('thrust')}
          onMouseDown={handleTouchStart('thrust')}
          onMouseUp={handleTouchEnd('thrust')}
          onMouseLeave={handleTouchEnd('thrust')}
        >
          THRUST
        </button>
        <button
          className={`control-btn fire-btn ${activeButtons.has('fire') ? 'active' : ''}`}
          onTouchStart={handleTouchStart('fire')}
          onTouchEnd={handleTouchEnd('fire')}
          onTouchCancel={handleTouchEnd('fire')}
          onMouseDown={handleTouchStart('fire')}
          onMouseUp={handleTouchEnd('fire')}
          onMouseLeave={handleTouchEnd('fire')}
        >
          FIRE
        </button>
      </div>
    </div>
  );
}

export default GameController;
