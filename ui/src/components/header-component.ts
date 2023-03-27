import { LitElement, html, css } from 'lit';
import { customElement } from 'lit/decorators.js';

@customElement('header-component')
class HeaderComponent extends LitElement {
  render() {
    return html`
      <h1>HoloScan</h1>
    `;
  }

  static styles = css`
    :host {
        text-align: left;
        color: #1d6fa5;
        box-shadow: 1px 1px #1d6fa5;
        margin: 10px;

        display: flex;
    }

  `;
}