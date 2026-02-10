package com.panthersecurity.sample

import android.os.Bundle
import android.view.LayoutInflater
import android.widget.ArrayAdapter
import android.widget.Button
import android.widget.EditText
import android.widget.LinearLayout
import android.widget.SeekBar
import android.widget.Spinner
import android.widget.TextView
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import androidx.appcompat.widget.SwitchCompat
import androidx.lifecycle.lifecycleScope
import com.panthersecurity.sample.data.PolicyRepositoryImpl
import com.panthersecurity.sample.data.TelemetryRepositoryImpl
import com.panthersecurity.sample.domain.Decision
import com.panthersecurity.sample.domain.FetchPolicyUseCase
import com.panthersecurity.sample.domain.IntegritySignals
import com.panthersecurity.sample.domain.SendTelemetryUseCase
import com.panthersecurity.sample.presentation.MainViewModel
import com.panthersecurity.sample.sdk.PantherSecurityConfig
import com.panthersecurity.sample.sdk.PantherSecuritySdk
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {
    private lateinit var viewModel: MainViewModel

    private var signalJailbreak = false
    private var signalRoot = false
    private var signalDebugger = false
    private var signalHooking = false
    private var signalProxy = false
    private var attestation: String? = null
    private var riskScore: Int = 0

    private var currentAction: String = "login"

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val sdk = PantherSecuritySdk(
            PantherSecurityConfig(
                baseUrl = "http://localhost:8082",
                appId = "fintech.mobile",
                appVersion = "1.0.0",
                env = "local",
                apiToken = null
            )
        )

        viewModel = MainViewModel(
            fetchPolicy = FetchPolicyUseCase(PolicyRepositoryImpl(sdk)),
            sendTelemetry = SendTelemetryUseCase(TelemetryRepositoryImpl(sdk))
        )

        val loginContainer = findViewById<LinearLayout>(R.id.loginContainer)
        val homeContainer = findViewById<LinearLayout>(R.id.homeContainer)

        val loginSecurity = findViewById<TextView>(R.id.loginSecurity)
        val loginMessage = findViewById<TextView>(R.id.loginMessage)
        val signInButton = findViewById<Button>(R.id.signIn)

        val statusView = findViewById<TextView>(R.id.status)
        val policyView = findViewById<TextView>(R.id.policy)
        val decisionView = findViewById<TextView>(R.id.decision)
        val validationsView = findViewById<TextView>(R.id.validations)
        val actionResultView = findViewById<TextView>(R.id.actionResult)
        val cardNumberView = findViewById<TextView>(R.id.cardNumber)

        val fetchPolicyButton = findViewById<Button>(R.id.fetchPolicy)
        val sendTelemetryButton = findViewById<Button>(R.id.sendTelemetry)
        val openSecurityLabButton = findViewById<Button>(R.id.openSecurityLab)

        signInButton.setOnClickListener {
            lifecycleScope.launch {
                val result = viewModel.evaluateAction("login", currentSignals(), attestation, riskScore)
                loginSecurity.text = "Security check: ${result.decision}"
                loginMessage.text = result.validations
                when (result.decision) {
                    Decision.ALLOW -> {
                        loginContainer.visibility = LinearLayout.GONE
                        homeContainer.visibility = LinearLayout.VISIBLE
                        viewModel.loadPolicy()
                        statusView.text = "Status: ${viewModel.status}"
                        policyView.text = "Policy: ${viewModel.policy?.policyId ?: "-"}"
                    }
                    Decision.STEP_UP -> showStepUpDialog("Login") {
                        loginContainer.visibility = LinearLayout.GONE
                        homeContainer.visibility = LinearLayout.VISIBLE
                        viewModel.loadPolicy()
                        statusView.text = "Status: ${viewModel.status}"
                        policyView.text = "Policy: ${viewModel.policy?.policyId ?: "-"}"
                    }
                    Decision.DEGRADE -> {
                        loginMessage.text = "Limited access"
                    }
                    Decision.DENY -> {
                        loginMessage.text = "Login blocked"
                    }
                }
                viewModel.sendActionTelemetry("login", currentSignals())
            }
        }

        fetchPolicyButton.setOnClickListener {
            lifecycleScope.launch {
                viewModel.loadPolicy()
                statusView.text = "Status: ${viewModel.status}"
                policyView.text = "Policy: ${viewModel.policy?.policyId ?: "-"}"
            }
        }

        sendTelemetryButton.setOnClickListener {
            lifecycleScope.launch {
                viewModel.sendActionTelemetry(currentAction, currentSignals())
                statusView.text = "Status: ${viewModel.status}"
            }
        }

        findViewById<Button>(R.id.actionViewCard).setOnClickListener {
            runAction("view_card", decisionView, validationsView, statusView, actionResultView) {
                cardNumberView.text = "4987 0312 9988 1200"
            }
            if (decisionView.text.contains("DEGRADE")) {
                cardNumberView.text = "4987 •••• •••• 1200"
            }
        }

        findViewById<Button>(R.id.actionTransfer).setOnClickListener {
            runAction("transfer", decisionView, validationsView, statusView, actionResultView) {
                actionResultView.text = "Transfer completed"
            }
        }

        findViewById<Button>(R.id.actionAddBeneficiary).setOnClickListener {
            runAction("add_beneficiary", decisionView, validationsView, statusView, actionResultView) {
                actionResultView.text = "Beneficiary added"
            }
        }

        findViewById<Button>(R.id.actionChangePassword).setOnClickListener {
            runAction("change_password", decisionView, validationsView, statusView, actionResultView) {
                actionResultView.text = "Password changed"
            }
        }

        openSecurityLabButton.setOnClickListener {
            openSecurityLab()
        }
    }

    private fun runAction(
        action: String,
        decisionView: TextView,
        validationsView: TextView,
        statusView: TextView,
        actionResultView: TextView,
        onAllowed: () -> Unit
    ) {
        currentAction = action
        lifecycleScope.launch {
            val result = viewModel.evaluateAction(action, currentSignals(), attestation, riskScore)
            decisionView.text = "Decision: ${result.decision}"
            validationsView.text = "Validations: ${result.validations}"

            when (result.decision) {
                Decision.ALLOW -> {
                    statusView.text = "Status: allowed"
                    onAllowed()
                }
                Decision.STEP_UP -> {
                    statusView.text = "Status: step-up required"
                    showStepUpDialog(action) {
                        onAllowed()
                        statusView.text = "Status: step-up completed"
                    }
                }
                Decision.DEGRADE -> {
                    statusView.text = "Status: degraded"
                    actionResultView.text = "Limited view due to risk"
                }
                Decision.DENY -> {
                    statusView.text = "Status: denied"
                    actionResultView.text = "Action blocked by security policy"
                }
            }

            viewModel.sendActionTelemetry(action, currentSignals())
        }
    }

    private fun openSecurityLab() {
        val view = LayoutInflater.from(this).inflate(R.layout.dialog_security_lab, null)

        val jailbreakSwitch = view.findViewById<SwitchCompat>(R.id.labJailbreak)
        val rootSwitch = view.findViewById<SwitchCompat>(R.id.labRoot)
        val debuggerSwitch = view.findViewById<SwitchCompat>(R.id.labDebugger)
        val hookingSwitch = view.findViewById<SwitchCompat>(R.id.labHooking)
        val proxySwitch = view.findViewById<SwitchCompat>(R.id.labProxy)
        val attestationSpinner = view.findViewById<Spinner>(R.id.labAttestation)
        val riskLabel = view.findViewById<TextView>(R.id.labRiskLabel)
        val riskSeek = view.findViewById<SeekBar>(R.id.labRiskSeek)

        jailbreakSwitch.isChecked = signalJailbreak
        rootSwitch.isChecked = signalRoot
        debuggerSwitch.isChecked = signalDebugger
        hookingSwitch.isChecked = signalHooking
        proxySwitch.isChecked = signalProxy

        val attestationValues = listOf("none", "pass", "fail", "unknown")
        attestationSpinner.adapter = ArrayAdapter(this, android.R.layout.simple_spinner_dropdown_item, attestationValues)
        attestationSpinner.setSelection(attestationValues.indexOf(attestation ?: "none").coerceAtLeast(0))

        riskSeek.progress = riskScore
        riskLabel.text = "Risk Score: $riskScore"

        riskSeek.setOnSeekBarChangeListener(object : SeekBar.OnSeekBarChangeListener {
            override fun onProgressChanged(seekBar: SeekBar?, progress: Int, fromUser: Boolean) {
                riskLabel.text = "Risk Score: $progress"
            }

            override fun onStartTrackingTouch(seekBar: SeekBar?) = Unit
            override fun onStopTrackingTouch(seekBar: SeekBar?) = Unit
        })

        AlertDialog.Builder(this)
            .setTitle("Security Lab (Demo Mode)")
            .setView(view)
            .setPositiveButton("Save") { _, _ ->
                signalJailbreak = jailbreakSwitch.isChecked
                signalRoot = rootSwitch.isChecked
                signalDebugger = debuggerSwitch.isChecked
                signalHooking = hookingSwitch.isChecked
                signalProxy = proxySwitch.isChecked
                attestation = attestationSpinner.selectedItem?.toString()?.takeIf { it != "none" }
                riskScore = riskSeek.progress
            }
            .setNegativeButton("Cancel", null)
            .show()
    }

    private fun showStepUpDialog(action: String, onConfirm: () -> Unit) {
        AlertDialog.Builder(this)
            .setTitle("Step-Up Required")
            .setMessage("Additional verification required for $action")
            .setPositiveButton("Verify") { _, _ -> onConfirm() }
            .setNegativeButton("Cancel", null)
            .show()
    }

    private fun currentSignals(): IntegritySignals {
        return IntegritySignals(
            jailbreak = signalJailbreak,
            root = signalRoot,
            debugger = signalDebugger,
            hooking = signalHooking,
            proxyDetected = signalProxy
        )
    }
}
