
import { useState } from "react";
import { useWallet } from "@solana/wallet-adapter-react";
import {
  Home,
  UserRound,
  BriefcaseBusiness,
  Store,
  ClipboardList,
  LayoutDashboard,
  Menu,
  X,
  Wallet
} from "lucide-react";

import WalletButton from "./Components/WalletButton";
import WalletInfo from "./Components/WalletInfo";

import RegisterCustomer from "./Pages/RegisterCustomer";
import RegisterProvider from "./Pages/RegisterProvider";
import Marketplace from "./Pages/MarketPlace";
import ProviderDashboard from "./Pages/ProviderDashboard";
import CustomerDashboard from "./Pages/CustomerDashboard";

const navigation = [
  { id: "home", label: "Overview", icon: Home },
  { id: "customer", label: "Register Customer", icon: UserRound },
  { id: "provider", label: "Register Provider", icon: BriefcaseBusiness },
  { id: "marketplace", label: "Marketplace", icon: Store },
  { id: "customerDashboard", label: "My Bookings", icon: ClipboardList },
  { id: "providerDashboard", label: "Provider Dashboard", icon: LayoutDashboard }
];

function App() {
  const { connected, publicKey } = useWallet();
  const [page, setPage] = useState("home");
  const [sidebarOpen, setSidebarOpen] = useState(false);

  const currentPage = navigation.find((item) => item.id === page);

  function navigateTo(id) {
    setPage(id);
    setSidebarOpen(false);
  }

  if (!connected || !publicKey) {
    return (
      <div className="connect-screen">
        <div className="connect-background-shape shape-one" />
        <div className="connect-background-shape shape-two" />

        <div className="connect-card">
          <div className="connect-brand-icon">
            <BriefcaseBusiness size={35} />
          </div>

          <span className="connect-eyebrow">WELCOME TO</span>
          <h1>TrustLance</h1>

          <div className="connect-divider" />

          <h2>Connect your wallet to continue</h2>
          <p className="connect-help">
            Connect your wallet to book services and manage your jobs securely.
          </p>

          <div className="connect-wallet-button">
            <WalletButton />
          </div>

          <div className="connect-security">
            <Wallet size={15} />
            <span>Your wallet remains under your control.</span>
          </div>
        </div>

      </div>
    );
  }

  return (
    <div className="app-shell">
      <aside className={`sidebar ${sidebarOpen ? "sidebar-open" : ""}`}>
        <div className="brand">
          <div className="brand-icon">
            <BriefcaseBusiness size={23} />
          </div>
          <div>
            <h2>TrustLance</h2>
            <span>SERVICE MARKETPLACE</span>
          </div>
          <button
            className="mobile-close"
            onClick={() => setSidebarOpen(false)}
          >
            <X size={20} />
          </button>
        </div>

        <div className="sidebar-section-label">WORKSPACE</div>

        <nav className="sidebar-nav">
          {navigation.map((item) => {
            const Icon = item.icon;

            return (
              <button
                key={item.id}
                className={`nav-item ${page === item.id ? "active" : ""}`}
                onClick={() => navigateTo(item.id)}
              >
                <Icon size={19} />
                <span>{item.label}</span>
              </button>
            );
          })}
        </nav>

      </aside>

      {sidebarOpen && (
        <button
          className="sidebar-overlay"
          onClick={() => setSidebarOpen(false)}
        />
      )}

      <main className="main-area">
        <header className="topbar">
          <div className="topbar-left">
            <button
              className="mobile-menu"
              onClick={() => setSidebarOpen(true)}
            >
              <Menu size={22} />
            </button>
            <div>
              <span className="breadcrumb">TRUSTLANCE / WORKSPACE</span>
              <h1>{currentPage?.label || "Overview"}</h1>
            </div>
          </div>

          <div className="topbar-wallet">
            <Wallet size={17} />
            <WalletButton />
          </div>
        </header>

        <section className="content-area">
          <div className="wallet-summary">
            <WalletInfo />
          </div>

          <div className="page-content" key={page}>
            {page === "home" && (
              <div className="welcome-section">
                <div className="welcome-card">
                  <div className="welcome-copy">
                    <span className="eyebrow">SERVICE MARKETPLACE</span>
                    <h2>Services made simple. Payments made secure.</h2>
                    <p>
                      Discover service providers, book jobs, and manage
                      payments securely.
                    </p>
                    <button
                      className="primary-button"
                      onClick={() => navigateTo("marketplace")}
                    >
                      Explore Marketplace
                    </button>
                  </div>
                  <div className="welcome-art">
                    <BriefcaseBusiness size={100} strokeWidth={1.2} />
                  </div>
                </div>

                <div className="section-heading">
                  <div>
                    <h2>Get started</h2>
                    <p>Choose what you want to do on TrustLance.</p>
                  </div>
                </div>

                <div className="quick-actions">
                  <button onClick={() => navigateTo("customer")} className="action-card">
                    <UserRound size={24} />
                    <h3>Join as Customer</h3>
                    <p>Register and start booking services.</p>
                  </button>

                  <button onClick={() => navigateTo("provider")} className="action-card">
                    <BriefcaseBusiness size={24} />
                    <h3>Join as Provider</h3>
                    <p>List your service and receive bookings.</p>
                  </button>

                  <button onClick={() => navigateTo("marketplace")} className="action-card">
                    <Store size={24} />
                    <h3>Explore Services</h3>
                    <p>Browse available service providers.</p>
                  </button>
                </div>
              </div>
            )}

            {page === "customer" && <RegisterCustomer />}
            {page === "provider" && <RegisterProvider />}
            {page === "marketplace" && <Marketplace />}
            {page === "customerDashboard" && <CustomerDashboard />}
            {page === "providerDashboard" && <ProviderDashboard />}
          </div>
        </section>
      </main>
    </div>
  );
}

export default App;
