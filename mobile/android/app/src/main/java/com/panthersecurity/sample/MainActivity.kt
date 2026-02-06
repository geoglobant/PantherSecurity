package com.panthersecurity.sample

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity
import androidx.lifecycle.lifecycleScope
import com.panthersecurity.sample.data.PolicyRepositoryImpl
import com.panthersecurity.sample.data.TelemetryRepositoryImpl
import com.panthersecurity.sample.domain.FetchPolicyUseCase
import com.panthersecurity.sample.domain.SendTelemetryUseCase
import com.panthersecurity.sample.presentation.MainViewModel
import com.panthersecurity.sample.sdk.PantherSecurityConfig
import com.panthersecurity.sample.sdk.PantherSecuritySdk
import kotlinx.coroutines.launch

class MainActivity : AppCompatActivity() {
    private lateinit var viewModel: MainViewModel

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

        val statusView = findViewById<TextView>(R.id.status)
        lifecycleScope.launch {
            viewModel.loadPolicy()
            statusView.text = "Status: ${viewModel.status}"
        }

        lifecycleScope.launch {
            viewModel.sendLoginTelemetry()
            statusView.text = "Status: ${viewModel.status}"
        }
    }
}
