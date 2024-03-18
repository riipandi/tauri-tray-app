import { clx } from '@/utils/helpers'

interface TrafficLightProps {
  handleMinimize: () => void
  handleMaximize: () => void
  handleClose: () => void
  isFocused: boolean
}

export default function TrafficLight(props: TrafficLightProps) {
  return (
    <div class={clx('traffic-lights', props.isFocused && 'focus')}>
      <button type="button" class="traffic-light traffic-light-close" onClick={props.handleClose}>
        <svg width="7" height="7" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            stroke="#000"
            stroke-width="1.2"
            stroke-linecap="round"
            d="M1.182 5.99L5.99 1.182M5.99 6.132L1.182 1.323"
          />
        </svg>
      </button>
      <button
        type="button"
        class="traffic-light traffic-light-minimize"
        onClick={props.handleMinimize}
      >
        <svg width="7" height="2" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path stroke="#000" stroke-width="1.2" stroke-linecap="round" d="M.61.703h5.8" />
        </svg>
      </button>
      <button
        type="button"
        class="traffic-light traffic-light-maximize"
        onClick={props.handleMaximize}
      >
        <svg width="8" height="7" fill="none" xmlns="http://www.w3.org/2000/svg">
          <path
            stroke="#000"
            stroke-width="1.2"
            stroke-linecap="round"
            d="M1.1 3.4h5.8M3.9 6.4V.6"
          />
        </svg>
      </button>
    </div>
  )
}
