import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';

/**
 * An example element.
 *
 * @slot - This element has a slot
 * @csspart button - The button
 */
@customElement('theme-selector')
export class ThemeSelector extends LitElement {
  /**
   * The number of times the button has been clicked.
   */
  @property({ type: String })
  themeName = 'dark';

  render() {
    return html`
      <button @click=${this._onClick} part="button">
        count is ${this.themeName}
      </button>
    `;
  }

  private _onClick() {
    const root = document.querySelector(':root');
    const currentTheme = root?.getAttribute('theme');
    root?.setAttribute('theme', currentTheme === 'light' ? 'dark' : 'light');
    this.themeName = 'dark';
  }

  static styles = css`
    button {
      color: var(--button-text-color);
      border-radius: 8px;
      border: 1px solid transparent;
      padding: 0.6em 1.2em;
      font-size: 1em;
      font-weight: 500;
      font-family: inherit;
      background-color: var(--button-background-color);
      cursor: pointer;
      transition: border-color 0.25s;
    }
    button:hover {
      border-color: #646cff;
    }
    button:focus,
    button:focus-visible {
      outline: 4px auto -webkit-focus-ring-color;
    }
  `;
}
