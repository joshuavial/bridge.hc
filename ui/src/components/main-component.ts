import { LitElement, html, css } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('main-component')
class MainComponent extends LitElement {
  render() {
    return html`
      <span>Transactions Log</span>
    `;
  }

  static styles = css`
    :host {
      background-color: #f3f3f3;
      flex-grow: 7;
      display: flex;
    }
  `;
}